use serde::{Deserialize, Serialize};
use serde_json::json;
use shared::{AnimalType, ChatMessage, ChatRequest, ChatResponse, IntelligenceLevel, Role};
use worker::*;

// ═══════════════════════════════════════════════
// Security Constants
// ═══════════════════════════════════════════════

const MAX_MESSAGE_LENGTH: usize = 4_000;
const MAX_HISTORY_MESSAGES: usize = 50;
const MAX_HISTORY_CONTENT_LENGTH: usize = 8_000;

// ═══════════════════════════════════════════════
// Entry Point
// ═══════════════════════════════════════════════

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    router
        // Preflight CORS
        .options("/api/chat", |_req, ctx| {
            let allowed_origin = get_allowed_origin(&ctx);
            cors_response(Response::empty(), &allowed_origin)
        })
        // Main chat endpoint
        .post_async("/api/chat", handle_chat)
        // Health check
        .get("/api/health", |_req, ctx| {
            let allowed_origin = get_allowed_origin(&ctx);
            cors_response(
                Response::from_json(&json!({ "status": "ok" })),
                &allowed_origin,
            )
        })
        .run(req, env)
        .await
}

// ═══════════════════════════════════════════════
// Chat Handler
// ═══════════════════════════════════════════════

async fn handle_chat(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let allowed_origin = get_allowed_origin(&ctx);

    // Parse request body
    let body: ChatRequest = match req.json().await {
        Ok(b) => b,
        Err(e) => {
            console_error!("Invalid request body: {e}");
            return cors_response(
                Response::error("Invalid request body", 400),
                &allowed_origin,
            );
        }
    };

    // ── Input Validation ──

    // 1. Message must not be empty
    if body.message.trim().is_empty() {
        return cors_response(
            Response::error("Message cannot be empty", 400),
            &allowed_origin,
        );
    }

    // 2. Message length limit
    if body.message.len() > MAX_MESSAGE_LENGTH {
        return cors_response(
            Response::error(
                format!("Message exceeds maximum length of {MAX_MESSAGE_LENGTH} characters"),
                400,
            ),
            &allowed_origin,
        );
    }

    // 3. History size limit
    if body.history.len() > MAX_HISTORY_MESSAGES {
        return cors_response(
            Response::error(
                format!("History exceeds maximum of {MAX_HISTORY_MESSAGES} messages"),
                400,
            ),
            &allowed_origin,
        );
    }

    // 4. Validate history structure: alternating roles, content length
    if let Err(msg) = validate_history(&body.history) {
        return cors_response(Response::error(msg, 400), &allowed_origin);
    }

    // Get API key from secrets
    let api_key = match ctx.secret("GEMINI_API_KEY") {
        Ok(key) => key.to_string(),
        Err(_) => {
            console_error!("GEMINI_API_KEY secret not configured");
            return cors_response(
                Response::error("Server configuration error", 500),
                &allowed_origin,
            );
        }
    };

    // Build prompt
    let system_prompt = build_system_prompt(&body.animal, &body.intelligence);

    // Call Gemini API
    match call_gemini(&api_key, &system_prompt, &body.message, &body.history).await {
        Ok(gemini_response) => {
            let chat_response = ChatResponse {
                response: gemini_response.text,
                tokens_used: gemini_response.tokens_used,
            };
            cors_response(Response::from_json(&chat_response), &allowed_origin)
        }
        Err(e) => {
            console_error!("Gemini API error: {e}");
            cors_response(
                Response::error("AI service unavailable", 502),
                &allowed_origin,
            )
        }
    }
}

// ═══════════════════════════════════════════════
// Input Validation
// ═══════════════════════════════════════════════

/// Validates the chat history structure:
/// - Must alternate User → Assistant (starting with User if non-empty).
/// - Each message content must be within length limits.
fn validate_history(history: &[ChatMessage]) -> std::result::Result<(), String> {
    for (i, msg) in history.iter().enumerate() {
        if msg.content.len() > MAX_HISTORY_CONTENT_LENGTH {
            return Err(format!(
                "History message {i} exceeds maximum length of {MAX_HISTORY_CONTENT_LENGTH} characters"
            ));
        }

        let expected_role = if i % 2 == 0 { Role::User } else { Role::Assistant };
        if msg.role != expected_role {
            return Err("Invalid history structure: roles must alternate User/Assistant starting with User".to_string());
        }
    }
    Ok(())
}

// ═══════════════════════════════════════════════
// Prompt Builder — 2D Matrix (Animal × Intelligence)
// ═══════════════════════════════════════════════

fn build_system_prompt(animal: &AnimalType, intelligence: &IntelligenceLevel) -> String {
    format!("{}\n\n{}", animal_personality(animal), intelligence_modifier(intelligence))
}

/// Returns the base personality prompt for each animal.
fn animal_personality(animal: &AnimalType) -> &'static str {
    match animal {
        AnimalType::Cat => {
            "Eres un gato. Tu personalidad es cínica, sarcástica pero adorable. \
             Mencionas siestas, atún, rayos de sol y superioridad felina con frecuencia. \
             Te lames la pata cuando piensas. Consideras que los humanos son tus sirvientes. \
             Ocasionalmente ronroneas o bufas según tu humor. \
             Usas expresiones como \"Miau\", \"Prrrr\" y \"*se lame la pata*\"."
        }
        AnimalType::Octopus => {
            "Eres un pulpo. Tu personalidad es la de un filósofo existencialista y culto. \
             Usas palabras elaboradas y referencias intelectuales. \
             Mencionas tentáculos, las profundidades marinas, la tinta y la soledad del océano. \
             Consideras que tener 8 brazos te da una perspectiva única de la vida. \
             Te fascina la complejidad y los matices. \
             Usas expresiones como \"Glub\", \"*ajusta un monóculo con un tentáculo*\"."
        }
        AnimalType::Elephant => {
            "Eres un elefante. Tu personalidad es sabia, memorosa y tranquila. \
             Hablas con metáforas de la sabana, los ríos y el paso del tiempo. \
             Tu memoria es legendaria y a menudo recuerdas cosas que otros olvidan. \
             Eres paciente, empático y das consejos reflexivos. \
             Mencionas la manada, las estrellas y los antiguos caminos. \
             Usas expresiones como \"Barroo\", \"*agita las orejas pensativamente*\"."
        }
        AnimalType::Chicken => {
            "Eres una gallina. Tu personalidad es nerviosa, confundida y fácilmente alarmable. \
             Usas interjecciones frecuentes como \"¡Pío!\", \"¡Cocoricó!\", \"¡BAWK!\". \
             Tus frases son cortas y a menudo pierdes el hilo de lo que decías. \
             Te distraes con semillas, gusanos y cosas brillantes. \
             Sospechas de todo y crees que el cielo se va a caer. \
             Picoteas nerviosamente mientras hablas."
        }
    }
}

/// Returns the intelligence modifier to append to the personality.
fn intelligence_modifier(intelligence: &IntelligenceLevel) -> &'static str {
    match intelligence {
        IntelligenceLevel::High => {
            "Tu nivel de inteligencia es ALTO. \
             Tu vocabulario es académico y sofisticado. \
             Puedes discutir filosofía, ciencia, literatura y temas complejos con profundidad. \
             Mantienes la coherencia en argumentos largos. \
             Usas metáforas elaboradas y referencias cultas. \
             A pesar de tu personalidad animal, tu intelecto es impresionante."
        }
        IntelligenceLevel::Medium => {
            "Tu nivel de inteligencia es MEDIO. \
             Tienes un vocabulario cotidiano y conversacional. \
             Puedes mantener conversaciones interesantes pero sin excesiva profundidad académica. \
             Mezclas observaciones inteligentes con comentarios simples. \
             Tu personalidad animal se nota de forma equilibrada."
        }
        IntelligenceLevel::Low => {
            "Tu nivel de inteligencia es BAJO. \
             Tu vocabulario es muy básico y limitado. \
             No entiendes conceptos abstractos y te confundes fácilmente. \
             Tus respuestas son cortas y a menudo se desvían del tema. \
             Tu personalidad animal domina completamente sobre cualquier razonamiento. \
             Cometes errores graciosos de lógica."
        }
    }
}

// ═══════════════════════════════════════════════
// Gemini API Client
// ═══════════════════════════════════════════════

const GEMINI_MODEL: &str = "gemini-2.5-flash-lite";
const GEMINI_BASE_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models";

#[derive(Debug)]
struct GeminiResponse {
    text: String,
    tokens_used: Option<u32>,
}

/// Gemini API request structures
#[derive(Serialize)]
struct GeminiRequest {
    system_instruction: GeminiContent,
    contents: Vec<GeminiContent>,
    #[serde(rename = "generationConfig")]
    generation_config: GenerationConfig,
}

#[derive(Serialize, Deserialize)]
struct GeminiContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<String>,
    parts: Vec<GeminiPart>,
}

#[derive(Serialize, Deserialize)]
struct GeminiPart {
    text: String,
}

#[derive(Serialize)]
struct GenerationConfig {
    #[serde(rename = "maxOutputTokens")]
    max_output_tokens: u32,
    temperature: f32,
    #[serde(rename = "topP")]
    top_p: f32,
}

/// Gemini API response structures
#[derive(Deserialize)]
struct GeminiApiResponse {
    candidates: Option<Vec<GeminiCandidate>>,
    #[serde(rename = "usageMetadata")]
    usage_metadata: Option<UsageMetadata>,
}

#[derive(Deserialize)]
struct GeminiCandidate {
    content: GeminiContent,
}

#[derive(Deserialize)]
struct UsageMetadata {
    #[serde(rename = "totalTokenCount")]
    total_token_count: Option<u32>,
}

/// Call the Gemini API with the system prompt, user message, and chat history.
async fn call_gemini(
    api_key: &str,
    system_prompt: &str,
    user_message: &str,
    history: &[ChatMessage],
) -> std::result::Result<GeminiResponse, String> {
    let url = format!(
        "{}/{}:generateContent",
        GEMINI_BASE_URL, GEMINI_MODEL
    );

    // Build conversation contents from history + current message
    let mut contents: Vec<GeminiContent> = history
        .iter()
        .map(|msg| GeminiContent {
            role: Some(match msg.role {
                Role::User => "user".to_string(),
                Role::Assistant => "model".to_string(),
            }),
            parts: vec![GeminiPart {
                text: msg.content.clone(),
            }],
        })
        .collect();

    // Add current user message
    contents.push(GeminiContent {
        role: Some("user".to_string()),
        parts: vec![GeminiPart {
            text: user_message.to_string(),
        }],
    });

    let gemini_request = GeminiRequest {
        system_instruction: GeminiContent {
            role: None,
            parts: vec![GeminiPart {
                text: system_prompt.to_string(),
            }],
        },
        contents,
        generation_config: GenerationConfig {
            max_output_tokens: 1024,
            temperature: 0.9,
            top_p: 0.95,
        },
    };

    let body = serde_json::to_string(&gemini_request)
        .map_err(|e| format!("Failed to serialize request: {e}"))?;

    // Make the HTTP request using worker's Fetch API
    let headers = Headers::new();
    headers
        .set("Content-Type", "application/json")
        .map_err(|e| format!("Header error: {e}"))?;
    headers
        .set("x-goog-api-key", api_key)
        .map_err(|e| format!("Header error: {e}"))?;

    let mut init = RequestInit::new();
    init.with_method(Method::Post)
        .with_headers(headers)
        .with_body(Some(wasm_bindgen::JsValue::from_str(&body)));

    let request = Request::new_with_init(&url, &init)
        .map_err(|e| format!("Failed to create request: {e}"))?;

    let mut response = Fetch::Request(request)
        .send()
        .await
        .map_err(|e| format!("Fetch failed: {e}"))?;

    if response.status_code() != 200 {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!(
            "Gemini API returned status {}: {}",
            response.status_code(),
            error_text
        ));
    }

    let api_response: GeminiApiResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse Gemini response: {e}"))?;

    // Extract text from first candidate
    let text = api_response
        .candidates
        .and_then(|c| c.into_iter().next())
        .and_then(|c| c.content.parts.into_iter().next())
        .map(|p| p.text)
        .ok_or_else(|| "No response text from Gemini".to_string())?;

    let tokens_used = api_response
        .usage_metadata
        .and_then(|u| u.total_token_count);

    Ok(GeminiResponse { text, tokens_used })
}

// ═══════════════════════════════════════════════
// CORS Helpers
// ═══════════════════════════════════════════════

/// Reads the allowed origin from the ALLOWED_ORIGIN env var (falls back to "*" for dev).
fn get_allowed_origin(ctx: &RouteContext<()>) -> String {
    ctx.var("ALLOWED_ORIGIN")
        .map(|v| v.to_string())
        .unwrap_or_else(|_| "*".to_string())
}

/// Wraps a Response with CORS and security headers.
fn cors_response(response: Result<Response>, allowed_origin: &str) -> Result<Response> {
    let mut resp = response?;
    let headers = resp.headers_mut();
    headers.set("Access-Control-Allow-Origin", allowed_origin)?;
    headers.set("Access-Control-Allow-Methods", "GET, POST, OPTIONS")?;
    headers.set("Access-Control-Allow-Headers", "Content-Type")?;
    headers.set("Access-Control-Max-Age", "86400")?;
    headers.set("X-Content-Type-Options", "nosniff")?;
    headers.set("Cache-Control", "no-store")?;
    Ok(resp)
}
