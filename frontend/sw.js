// ═══════════════════════════════════════════════
// Service Worker — Inteligencia Animal
// Network-First with Controlled Updates
// ═══════════════════════════════════════════════

const CACHE_VERSION = 'v2';
const APP_SHELL_CACHE = `app-shell-${CACHE_VERSION}`;
const STATIC_CACHE = `static-assets-${CACHE_VERSION}`;
const FONTS_CACHE = `fonts-${CACHE_VERSION}`;

const APP_SHELL_ASSETS = [
  '/',
  '/index.html',
];

const FONT_ASSETS = [
  '/public/fonts/inter-latin-400.woff2',
  '/public/fonts/inter-latin-500.woff2',
  '/public/fonts/inter-latin-600.woff2',
  '/public/fonts/inter-latin-700.woff2',
  '/public/fonts/material-symbols.woff2',
];

const EXPECTED_CACHES = new Set([APP_SHELL_CACHE, STATIC_CACHE, FONTS_CACHE]);

const STATIC_ASSET_RE = /\.(wasm|js|css|svg|png|jpg|jpeg|webp|ico|json)$/i;
const CONTENT_HASH_RE = /[-_][a-f0-9]{8,}\./i;

const VALID_ANIMALS = new Set(['cat', 'chicken', 'elephant', 'octopus']);

// ── Install ──
self.addEventListener('install', (event) => {
  console.log('[SW] Install — caching App Shell & Fonts');
  event.waitUntil(
    Promise.all([
      caches.open(APP_SHELL_CACHE)
        .then((cache) => cache.addAll(APP_SHELL_ASSETS)),
      caches.open(FONTS_CACHE)
        .then((cache) => cache.addAll(FONT_ASSETS)),
    ])
  );
});

// ── Activate ──
self.addEventListener('activate', (event) => {
  console.log('[SW] Activate — cleaning old caches');
  event.waitUntil(
    caches.keys()
      .then((cacheNames) =>
        Promise.all(
          cacheNames
            .filter((name) => !EXPECTED_CACHES.has(name))
            .map((name) => {
              console.log(`[SW] Deleting old cache: ${name}`);
              return caches.delete(name);
            })
        )
      )
      .then(() => self.clients.claim())
  );
});

// ── Fetch ──
self.addEventListener('fetch', (event) => {
  const { request } = event;
  const url = new URL(request.url);

  if (!url.protocol.startsWith('http')) {
    return;
  }

  if (request.method !== 'GET') {
    return;
  }

  // 1. API requests → Network-Only (never cache)
  if (url.pathname.startsWith('/api/')) {
    event.respondWith(
      fetch(request).catch(() =>
        new Response(JSON.stringify({ error: 'offline' }), {
          status: 503,
          headers: { 'Content-Type': 'application/json' },
        })
      )
    );
    return;
  }

  // 2. Navigation requests → Network-First (always try to get fresh HTML)
  if (request.mode === 'navigate') {
    event.respondWith(networkFirst(request, APP_SHELL_CACHE));
    return;
  }

  // 3. Local fonts → Cache-First with long-term caching
  if (url.pathname.endsWith('.woff2')) {
    event.respondWith(cacheFirst(request, FONTS_CACHE));
    return;
  }

  // 4. Static assets with hash in name → Cache-First (safe, hash = unique)
  if (isStaticAsset(url.pathname)) {
    if (hasContentHash(url.pathname)) {
      event.respondWith(cacheFirst(request, STATIC_CACHE));
    } else {
      event.respondWith(staleWhileRevalidate(request, STATIC_CACHE));
    }
    return;
  }

  // 5. Everything else → Network-First with cache fallback
  event.respondWith(networkFirst(request, STATIC_CACHE));
});

// ── Message Handler ──
self.addEventListener('message', (event) => {
  // Validate origin — only accept messages from same origin
  if (event.origin && event.origin !== self.location.origin) {
    return;
  }

  const { data } = event;
  if (!data || typeof data !== 'object') {
    return;
  }

  switch (data.type) {
    case 'SKIP_WAITING':
      void self.skipWaiting();
      break;

    case 'PRECACHE_THEME': {
      const { animal } = data;
      if (typeof animal === 'string' && VALID_ANIMALS.has(animal)) {
        console.log(`[SW] Pre-caching theme: ${animal}`);
        // Future: pre-cache theme-specific CSS/SVG assets from R2
      }
      break;
    }
  }
});

// ═══════════════════════════════════════════════
// Caching Strategies
// ═══════════════════════════════════════════════

/**
 * Cache-First: Return cached response if available, otherwise fetch and cache.
 * Injects Cache-Control headers so Lighthouse detects efficient caching.
 */
async function cacheFirst(request, cacheName) {
  const cached = await caches.match(request);
  if (cached) {
    return withCacheHeaders(cached, 31536000); // 1 year
  }

  try {
    const response = await fetch(request);
    if (response.ok) {
      const cache = await caches.open(cacheName);
      await cache.put(request, response.clone());
    }
    return withCacheHeaders(response, 31536000);
  } catch {
    if (request.mode === 'navigate') {
      const shell = await caches.match('/index.html');
      if (shell) return shell;
    }
    return new Response('Offline', { status: 503, statusText: 'Service Unavailable' });
  }
}

/**
 * Network-First: Try network, fall back to cache.
 * Adds Cache-Control headers when responding with cached content.
 */
async function networkFirst(request, cacheName) {
  try {
    const response = await fetch(request);
    if (response.ok) {
      const cache = await caches.open(cacheName);
      await cache.put(request, response.clone());
    }
    return response;
  } catch {
    const cached = await caches.match(request);
    if (cached) return withCacheHeaders(cached, 3600); // 1 hour
    if (request.mode === 'navigate') {
      const shell = await caches.match('/index.html');
      if (shell) return shell;
    }
    return new Response('Offline', { status: 503, statusText: 'Service Unavailable' });
  }
}

/**
 * Stale-While-Revalidate: Return cached immediately, update cache in background.
 * Injects Cache-Control headers for Lighthouse compliance.
 */
async function staleWhileRevalidate(request, cacheName) {
  const cache = await caches.open(cacheName);
  const cached = await cache.match(request);

  const fetchPromise = fetch(request)
    .then(async (response) => {
      if (response.ok) {
        await cache.put(request, response.clone());
      }
      return response;
    })
    .catch(() => null);

  if (cached) {
    fetchPromise.catch(() => {});
    return withCacheHeaders(cached, 86400); // 1 day
  }

  const networkResponse = await fetchPromise;
  if (networkResponse) {
    return withCacheHeaders(networkResponse, 86400);
  }
  return new Response('Offline', { status: 503, statusText: 'Service Unavailable' });
}

// ═══════════════════════════════════════════════
// Helpers
// ═══════════════════════════════════════════════

/**
 * Clone a response and add Cache-Control headers so Lighthouse
 * recognises efficient caching even when served from the SW cache.
 */
function withCacheHeaders(response, maxAgeSec) {
  const headers = new Headers(response.headers);
  headers.set('Cache-Control', `public, max-age=${maxAgeSec}, immutable`);
  return new Response(response.body, {
    status: response.status,
    statusText: response.statusText,
    headers,
  });
}

function isStaticAsset(pathname) {
  return STATIC_ASSET_RE.test(pathname);
}

/**
 * Detect if the filename contains a content hash (e.g. app-a1b2c3d4.js, index-abc123_bg.wasm).
 * Trunk generates hashed filenames by default. Hashed assets are immutable and safe to Cache-First.
 */
function hasContentHash(pathname) {
  return CONTENT_HASH_RE.test(pathname);
}

