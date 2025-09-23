use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <style>
                {"
                * {
                    margin: 0;
                    padding: 0;
                    box-sizing: border-box;
                }
                
                body {
                    font-family: 'Helvetica Neue', 'Arial', sans-serif;
                    background-color: #fafafa;
                }
                
                .container {
                    display: flex;
                    flex-direction: column;
                    min-height: 100vh;
                    width: 100%;
                }
                
                .main-content {
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    justify-content: center;
                    flex: 1;
                    padding: 2rem;
                }
                
                .content {
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    width: 100%;
                    max-width: 1200px;
                }
                
                .artwork {
                    width: 61.8%;
                    max-width: 800px;
                    max-height: 70vh;
                    height: auto;
                    object-fit: contain;
                    margin-bottom: 2.5rem;
                    border-radius: 2px;
                    filter: drop-shadow(0 8px 32px rgba(0,0,0,0.12));
                }
                
                .artwork-info {
                    text-align: center;
                    max-width: 50%;
                    margin-top: 0;
                    padding: 0 1rem;
                }
                
                .artist-name {
                    font-size: clamp(1.2rem, 3vw, 2rem);
                    font-weight: 200;
                    margin: 0 0 0.618rem 0;
                    color: #2c2c2c;
                    letter-spacing: 0.1em;
                    text-transform: uppercase;
                    font-family: 'Helvetica Neue', 'Arial', sans-serif;
                }
                
                .artwork-title {
                    font-size: clamp(0.9rem, 2vw, 1.2rem);
                    font-weight: 300;
                    margin: 0 0 0.382rem 0;
                    color: #666;
                    font-style: italic;
                    letter-spacing: 0.05em;
                }
                
                .artwork-year {
                    font-size: clamp(0.7rem, 1.2vw, 0.9rem);
                    margin: 0;
                    color: #888;
                    font-weight: 300;
                    letter-spacing: 0.1em;
                    text-transform: uppercase;
                }
                
                /* Large screens (desktop) */
                @media (min-width: 1200px) {
                    .main-content {
                        padding: 3rem;
                    }
                    
                    .artwork {
                        width: 70%;
                        max-width: 1000px;
                        max-height: 65vh;
                        margin-bottom: 3rem;
                    }
                    
                    .artwork-info {
                        max-width: 60%;
                        margin-top: 0;
                        padding: 0 2rem;
                    }
                }
                
                /* Medium screens (tablet) */
                @media (max-width: 768px) {
                    .main-content {
                        padding: 1.5rem;
                    }
                    
                    .artwork {
                        width: 80%;
                        margin-bottom: 2rem;
                    }
                    
                    .artwork-info {
                        max-width: 70%;
                        margin-top: 0;
                    }
                }
                
                /* Small screens (mobile) */
                @media (max-width: 480px) {
                    .main-content {
                        padding: 1rem;
                    }
                    
                    .artwork {
                        width: 90%;
                        margin-bottom: 1.5rem;
                    }
                    
                    .artwork-info {
                        max-width: 85%;
                        margin-top: 0;
                    }
                }
                
                /* Ultra-wide screens */
                @media (min-width: 1920px) {
                    .artwork {
                        width: 75%;
                        max-width: 1200px;
                        max-height: 60vh;
                        margin-bottom: 3.5rem;
                    }
                    
                    .artwork-info {
                        max-width: 65%;
                        margin-top: 0;
                        padding: 0 3rem;
                    }
                }
                
                /* Ultra-ultra-wide screens (4K+) */
                @media (min-width: 2560px) {
                    .artwork {
                        width: 80%;
                        max-width: 1400px;
                        max-height: 55vh;
                        margin-bottom: 4rem;
                    }
                    
                    .artwork-info {
                        max-width: 70%;
                        margin-top: 0;
                        padding: 0 4rem;
                    }
                }
                
                /* Footer styles */
                .footer {
                    background-color: #2c2c2c;
                    color: #e0e0e0;
                    padding: 2rem 0;
                    margin-top: auto;
                    text-align: center;
                    font-family: 'Helvetica Neue', 'Arial', sans-serif;
                    font-size: 0.875rem;
                    font-weight: 300;
                    letter-spacing: 0.05em;
                }
                
                .footer-content {
                    max-width: 1200px;
                    margin: 0 auto;
                    padding: 0 2rem;
                }
                
                .footer-intro {
                    margin-bottom: 1.5rem;
                    max-width: 800px;
                    margin-left: auto;
                    margin-right: auto;
                }
                
                .footer-description {
                    margin: 0;
                    font-size: 0.95rem;
                    line-height: 1.7;
                    opacity: 1;
                    font-weight: 300;
                    letter-spacing: 0.02em;
                    text-align: center;
                    max-width: 700px;
                    margin-left: auto;
                    margin-right: auto;
                }
                
                .footer-text {
                    margin: 0;
                    opacity: 0.9;
                    font-size: 0.8rem;
                    letter-spacing: 0.1em;
                    text-transform: uppercase;
                    font-weight: 400;
                }
                
                /* Responsive footer */
                @media (max-width: 768px) {
                    .footer {
                        padding: 1.5rem 0;
                        font-size: 0.8rem;
                    }
                    
                    .footer-content {
                        padding: 0 1rem;
                    }
                    
                    .footer-intro {
                        margin-bottom: 1rem;
                    }
                    
                    .footer-description {
                        font-size: 0.85rem;
                        line-height: 1.6;
                        letter-spacing: 0.01em;
                        max-width: 90%;
                    }
                    
                    .footer-text {
                        font-size: 0.75rem;
                        letter-spacing: 0.08em;
                    }
                }
                "}
            </style>
            <div class="container">
                <div class="main-content">
                    <div class="content">
                        <img 
                            src="/blue.jpg" 
                            alt="0xbase.ai" 
                            class="artwork"
                        />
                        <div class="artwork-info">
                            <h1 class="artist-name">
                                { "0xbase.ai" }
                            </h1>
                            <h2 class="artwork-title">
                                { "We focus in Base, LLM and Rust" }
                            </h2>
                            <p class="artwork-year">
                                { "2024" }
                            </p>
                        </div>
                    </div>
                </div>
                <footer class="footer">
                    <div class="footer-content">
                        <div class="footer-intro">
                            <p class="footer-description">
                                { "0xbase.ai is a crypto-punk driven decentralized organization, focused on building AI tools on Basechain. We have extreme requirements and security obsessions, demanding all code to be written in Rust and completely open source." }
                            </p>
                        </div>
                        <p class="footer-text">
                            { "© 2024 0xbase.ai • Inspired by MOMA" }
                        </p>
                    </div>
                </footer>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
