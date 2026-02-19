use leptos::prelude::*;
use shared::AnimalType;

/// Renders the SVG face of the selected animal as a watermark background.
#[component]
pub fn AnimalCard() -> impl IntoView {
    let animal = use_context::<Memo<AnimalType>>().expect("AnimalType context");

    let svg_view = move || {
        let svg_html = match animal.get() {
            AnimalType::Cat => CAT_SVG,
            AnimalType::Octopus => OCTOPUS_SVG,
            AnimalType::Elephant => ELEPHANT_SVG,
            AnimalType::Chicken => CHICKEN_SVG,
        };
        view! { <div inner_html=svg_html></div> }
    };

    view! {
        <div class="animal-watermark">
            {svg_view}
        </div>
    }
}

const CAT_SVG: &str = r#"<svg viewBox="0 0 200 200" fill="none" xmlns="http://www.w3.org/2000/svg">
<polygon points="45,80 60,20 85,70" fill="currentColor" opacity="0.9"/>
<polygon points="155,80 140,20 115,70" fill="currentColor" opacity="0.9"/>
<circle cx="100" cy="110" r="65" fill="currentColor" opacity="0.15"/>
<circle cx="100" cy="110" r="65" stroke="currentColor" stroke-width="3" fill="none"/>
<ellipse cx="78" cy="100" rx="8" ry="10" fill="currentColor"/>
<ellipse cx="122" cy="100" rx="8" ry="10" fill="currentColor"/>
<polygon points="100,115 94,122 106,122" fill="currentColor"/>
<line x1="30" y1="112" x2="75" y2="118" stroke="currentColor" stroke-width="2"/>
<line x1="30" y1="125" x2="75" y2="122" stroke="currentColor" stroke-width="2"/>
<line x1="30" y1="138" x2="75" y2="128" stroke="currentColor" stroke-width="2"/>
<line x1="170" y1="112" x2="125" y2="118" stroke="currentColor" stroke-width="2"/>
<line x1="170" y1="125" x2="125" y2="122" stroke="currentColor" stroke-width="2"/>
<line x1="170" y1="138" x2="125" y2="128" stroke="currentColor" stroke-width="2"/>
</svg>"#;

const OCTOPUS_SVG: &str = r#"<svg viewBox="0 0 200 200" fill="none" xmlns="http://www.w3.org/2000/svg">
<ellipse cx="100" cy="80" rx="60" ry="55" fill="currentColor" opacity="0.15"/>
<ellipse cx="100" cy="80" rx="60" ry="55" stroke="currentColor" stroke-width="3" fill="none"/>
<circle cx="80" cy="75" r="10" fill="currentColor"/>
<circle cx="120" cy="75" r="10" fill="currentColor"/>
<circle cx="83" cy="72" r="3" fill="white"/>
<circle cx="123" cy="72" r="3" fill="white"/>
<path d="M50,120 Q30,160 45,180" stroke="currentColor" stroke-width="3" fill="none" stroke-linecap="round"/>
<path d="M65,125 Q50,165 60,185" stroke="currentColor" stroke-width="3" fill="none" stroke-linecap="round"/>
<path d="M85,130 Q75,170 80,190" stroke="currentColor" stroke-width="3" fill="none" stroke-linecap="round"/>
<path d="M115,130 Q125,170 120,190" stroke="currentColor" stroke-width="3" fill="none" stroke-linecap="round"/>
<path d="M135,125 Q150,165 140,185" stroke="currentColor" stroke-width="3" fill="none" stroke-linecap="round"/>
<path d="M150,120 Q170,160 155,180" stroke="currentColor" stroke-width="3" fill="none" stroke-linecap="round"/>
</svg>"#;

const ELEPHANT_SVG: &str = r#"<svg viewBox="0 0 200 200" fill="none" xmlns="http://www.w3.org/2000/svg">
<ellipse cx="35" cy="90" rx="30" ry="40" fill="currentColor" opacity="0.1"/>
<ellipse cx="35" cy="90" rx="30" ry="40" stroke="currentColor" stroke-width="3" fill="none"/>
<ellipse cx="165" cy="90" rx="30" ry="40" fill="currentColor" opacity="0.1"/>
<ellipse cx="165" cy="90" rx="30" ry="40" stroke="currentColor" stroke-width="3" fill="none"/>
<circle cx="100" cy="90" r="55" fill="currentColor" opacity="0.15"/>
<circle cx="100" cy="90" r="55" stroke="currentColor" stroke-width="3" fill="none"/>
<circle cx="80" cy="80" r="6" fill="currentColor"/>
<circle cx="120" cy="80" r="6" fill="currentColor"/>
<path d="M100,110 Q100,140 90,160 Q85,170 90,180" stroke="currentColor" stroke-width="4" fill="none" stroke-linecap="round"/>
</svg>"#;

const CHICKEN_SVG: &str = r#"<svg viewBox="0 0 200 200" fill="none" xmlns="http://www.w3.org/2000/svg">
<circle cx="90" cy="30" r="12" fill="currentColor" opacity="0.6"/>
<circle cx="105" cy="25" r="14" fill="currentColor" opacity="0.6"/>
<circle cx="120" cy="32" r="11" fill="currentColor" opacity="0.6"/>
<circle cx="100" cy="90" r="55" fill="currentColor" opacity="0.15"/>
<circle cx="100" cy="90" r="55" stroke="currentColor" stroke-width="3" fill="none"/>
<circle cx="82" cy="80" r="7" fill="currentColor"/>
<circle cx="118" cy="80" r="7" fill="currentColor"/>
<circle cx="84" cy="78" r="2" fill="white"/>
<circle cx="120" cy="78" r="2" fill="white"/>
<polygon points="100,95 88,108 112,108" fill="currentColor" opacity="0.8"/>
<ellipse cx="100" cy="118" rx="8" ry="12" fill="currentColor" opacity="0.5"/>
</svg>"#;
