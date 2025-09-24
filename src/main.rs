use yew::prelude::*;
use web_sys::window;

#[function_component]
fn App() -> Html {
    let scroll_to_footer = Callback::from(|_| {
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                if let Some(footer) = document.get_element_by_id("footer") {
                    footer.scroll_into_view_with_bool(true);
                }
            }
        }
    });
    html! {
        <>
            <script>
                {"
                document.addEventListener('wheel', function(event) {
                    // Only trigger if scrolling down
                    if (event.deltaY > 0) {
                        const footer = document.getElementById('footer');
                        if (footer) {
                            footer.scrollIntoView({ behavior: 'smooth' });
                        }
                    }
                });
                "}
            </script>
            <style>
                {"
                * {
                    margin: 0;
                    padding: 0;
                    box-sizing: border-box;
                }
                
                body {
                    font-family: 'Helvetica Neue', 'Arial', sans-serif;
                    background: linear-gradient(45deg, 
                        #ffcccc, #ffd9cc, #ffe0e0, #ffe6cc, #ffffcc, #e0ffe0, 
                        #ccffff, #cce6ff);
                    background-size: 400% 400%;
                    animation: subtle-rainbow 8s ease-in-out infinite;
                    scroll-behavior: smooth;
                }
                
                html {
                    scroll-behavior: smooth;
                }
                
                /* Auto-scroll to footer on wheel event */
                body {
                    overflow-x: hidden;
                }
                
                .main-content {
                    height: 100vh;
                    overflow-y: auto;
                    scroll-snap-type: y mandatory;
                }
                
                .footer {
                    scroll-snap-align: start;
                }
                
                @keyframes subtle-rainbow {
                    0%, 100% { background-position: 0% 50%; }
                    50% { background-position: 100% 50%; }
                }
                
                .container {
                    display: flex;
                    flex-direction: column;
                    min-height: 100vh;
                    width: 100%;
                }
                
                .brand-header {
                    position: fixed;
                    top: 0;
                    left: 0;
                    z-index: 1000;
                    padding: 1.5rem 2rem;
                    font-family: 'Helvetica Neue', 'Arial', sans-serif;
                }
                
                .brand-text {
                    font-size: 1.2rem;
                    font-weight: 500;
                    letter-spacing: 0.05em;
                    color: #333;
                }
                
                .brand-0x {
                    background-image: linear-gradient(45deg, 
                        #cc0000, #cc6600, #cccc00, #00cc00, 
                        #00cccc, #0066cc, #6600cc, #cc00cc);
                    background-size: 400% 400%;
                    background-clip: text;
                    -webkit-background-clip: text;
                    -webkit-text-fill-color: transparent;
                    padding: 0.2rem 0 0.2rem 0.5rem;
                    font-weight: 600;
                    letter-spacing: 0.1em;
                    font-size: 1.1em;
                    animation: rainbow 3s ease-in-out infinite;
                }
                
                @keyframes rainbow {
                    0%, 100% { background-position: 0% 50%; }
                    50% { background-position: 100% 50%; }
                }
                
                .brand-base {
                    background-color: #0066cc;
                    color: white;
                    padding: 0.2rem 0;
                    font-weight: 600;
                    letter-spacing: 0.1em;
                    text-shadow: 1px 1px 0px #000;
                    font-size: 1.1em;
                }
                
                .brand-ai {
                    background-color: #8b5cf6;
                    color: white;
                    padding: 0.2rem 0.5rem 0.2rem 0;
                    font-weight: 600;
                    letter-spacing: 0.1em;
                    text-shadow: 1px 1px 0px #000;
                    font-size: 1.1em;
                }
                
                .main-content {
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    justify-content: center;
                    height: 100vh;
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
                    filter: drop-shadow(0 20px 80px rgba(0,0,0,0.25)) drop-shadow(0 8px 32px rgba(0,0,0,0.15)) drop-shadow(0 2px 8px rgba(0,0,0,0.1));
                }
                
                .artwork-info {
                    text-align: center;
                    max-width: 50%;
                    margin-top: 0;
                    padding: 0 1rem;
                }
                
                .artist-name {
                    font-size: clamp(0.9rem, 2vw, 1.2rem);
                    font-weight: 200;
                    margin: 0 0 0.618rem 0;
                    color: #2c2c2c;
                    letter-spacing: 0.1em;
                    text-transform: uppercase;
                    font-family: 'Helvetica Neue', 'Arial', sans-serif;
                }
                
                .artwork-title {
                    font-size: clamp(0.7rem, 1.5vw, 0.9rem);
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
                
                .artwork-link {
                    color: inherit;
                    text-decoration: none;
                    display: block;
                    transition: all 0.2s ease;
                }
                
                .artwork-link:hover {
                    transform: translateY(-2px);
                }
                
                .artwork-link:hover .artist-name,
                .artwork-link:hover .artwork-title,
                .artwork-link:hover .artwork-year {
                    color: #333;
                }
                
                /* Large screens (desktop) */
                @media (min-width: 1200px) {
                    .brand-header {
                        padding: 2rem 3rem;
                    }
                    
                    .brand-text {
                        font-size: 1.4rem;
                    }
                    
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
                    .brand-header {
                        padding: 1rem 1.5rem;
                    }
                    
                    .brand-text {
                        font-size: 1rem;
                    }
                    
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
                    min-height: 100vh;
                    margin-top: auto;
                    text-align: center;
                    font-family: 'Helvetica Neue', 'Arial', sans-serif;
                    font-size: 0.875rem;
                    font-weight: 300;
                    letter-spacing: 0.05em;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    padding: 2rem 0;
                }
                
                .footer-content {
                    max-width: 1000px;
                    margin: 0 auto;
                    padding: 4rem 3rem;
                    display: flex;
                    flex-direction: column;
                    gap: 3rem;
                    width: 100%;
                }
                
                .footer-intro {
                    margin-bottom: 0;
                    max-width: 800px;
                    margin-left: auto;
                    margin-right: auto;
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    gap: 1.5rem;
                }
                
                .footer-logo {
                    width: 64px;
                    height: 64px;
                    opacity: 0.9;
                    filter: brightness(1.2) contrast(1.1);
                }
                
                .footer-description {
                    margin: 0;
                    font-size: 1.1rem;
                    line-height: 1.8;
                    opacity: 1;
                    font-weight: 300;
                    letter-spacing: 0.02em;
                    text-align: center;
                    max-width: 800px;
                    margin-left: auto;
                    margin-right: auto;
                }
                
                .footer-projects {
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    gap: 0;
                    margin: 0;
                    padding: 2.5rem 0;
                    border-top: 1px solid rgba(255, 255, 255, 0.15);
                    border-bottom: 1px solid rgba(255, 255, 255, 0.15);
                }
                
                .project-link {
                    color: #e0e0e0;
                    text-decoration: none;
                    display: flex;
                    align-items: center;
                    gap: 1rem;
                    padding: 2rem 3rem;
                    transition: all 0.2s ease;
                    flex: 1;
                    max-width: 400px;
                    font-size: 1.1rem;
                    line-height: 1.5;
                    text-align: left;
                    min-height: 120px;
                }
                
                .project-link:hover {
                    color: #ffffff;
                }
                
                .project-logo {
                    width: 56px;
                    height: 56px;
                    opacity: 0.9;
                    transition: all 0.2s ease;
                    flex-shrink: 0;
                    align-self: flex-start;
                    margin-top: 0.5rem;
                }
                
                .project-link:hover .project-logo {
                    opacity: 1;
                    transform: scale(1.05);
                }
                
                .project-content {
                    display: flex;
                    flex-direction: column;
                    gap: 0.3rem;
                }
                
                .project-name {
                    font-size: 1.1rem;
                    font-weight: 600;
                    color: #ffffff;
                }
                
                .project-desc {
                    font-size: 0.9rem;
                    color: #cccccc;
                    font-weight: 400;
                }
                
                .project-detail {
                    font-size: 0.75rem;
                    color: #aaaaaa;
                    line-height: 1.4;
                    font-weight: 300;
                    max-width: 300px;
                }
                
                .project-link br {
                    margin: 0.3rem 0;
                }
                
                .project-divider {
                    width: 1px;
                    height: 80px;
                    background: linear-gradient(to bottom, 
                        transparent 0%, 
                        rgba(255, 255, 255, 0.1) 20%, 
                        rgba(255, 255, 255, 0.2) 50%, 
                        rgba(255, 255, 255, 0.1) 80%, 
                        transparent 100%);
                    flex-shrink: 0;
                    align-self: center;
                }
                
                .footer-text {
                    margin: 0;
                    opacity: 0.9;
                    font-size: 0.9rem;
                    letter-spacing: 0.1em;
                    text-transform: uppercase;
                    font-weight: 400;
                    padding-top: 1rem;
                }
                
                /* Responsive footer */
                @media (max-width: 768px) {
                    .footer {
                        min-height: 100vh;
                        font-size: 0.8rem;
                        padding: 1rem 0;
                    }
                    
                    .footer-content {
                        padding: 3rem 2rem;
                        gap: 2.5rem;
                    }
                    
                    .footer-intro {
                        margin-bottom: 0;
                        gap: 1rem;
                    }
                    
                    .footer-logo {
                        width: 56px;
                        height: 56px;
                    }
                    
                    .footer-description {
                        font-size: 1rem;
                        line-height: 1.7;
                        letter-spacing: 0.01em;
                        max-width: 90%;
                    }
                    
                    .footer-projects {
                        flex-direction: column;
                        gap: 2rem;
                        padding: 2rem 0;
                    }
                    
                    .project-link {
                        padding: 1.5rem 2rem;
                        max-width: none;
                        justify-content: center;
                        align-items: center;
                        gap: 1.5rem;
                        display: flex;
                        flex-direction: column;
                        text-align: center;
                    }
                    
                    .project-logo {
                        width: 48px;
                        height: 48px;
                        align-self: center;
                        margin-top: 0;
                    }
                    
                    .project-content {
                        align-items: center;
                        text-align: center;
                    }
                    
                    .project-name {
                        font-size: 1rem;
                    }
                    
                    .project-desc {
                        font-size: 0.8rem;
                    }
                    
                    .project-detail {
                        font-size: 0.7rem;
                        max-width: 280px;
                        text-align: center;
                    }
                    
                    .project-divider {
                        width: 60px;
                        height: 1px;
                        background: linear-gradient(to right, 
                            transparent 0%, 
                            rgba(255, 255, 255, 0.1) 20%, 
                            rgba(255, 255, 255, 0.2) 50%, 
                            rgba(255, 255, 255, 0.1) 80%, 
                            transparent 100%);
                    }
                    
                    .footer-text {
                        font-size: 0.8rem;
                        letter-spacing: 0.08em;
                    }
                }
                "}
            </style>
            <div class="container">
                <div class="brand-header">
                    <div class="brand-text">
                        <span class="brand-0x">{ "0x" }</span>
                        <span class="brand-base">{ "BASE" }</span>
                        <span class="brand-ai">{ ".AI" }</span>
                    </div>
                </div>
                <div class="main-content">
                    <div class="content">
                        <img 
                            src="/imgs/blue.jpg" 
                            alt="Blue Monochrome" 
                            class="artwork"
                            onclick={scroll_to_footer}
                            style="cursor: pointer;"
                        />
                        <div class="artwork-info">
                            <a href="https://www.moma.org/collection/works/80103" target="_blank" rel="noopener noreferrer" class="artwork-link">
                                <h1 class="artist-name">
                                    { "Yves Klein" }
                                </h1>
                                <h2 class="artwork-title">
                                    { "Blue Monochrome" }
                                </h2>
                                <p class="artwork-year">
                                    { "1961" }
                                </p>
                            </a>
                        </div>
                    </div>
                </div>
                <footer id="footer" class="footer">
                    <div class="footer-content">
                        <div class="footer-intro">
                            <img 
                                src="/imgs/logo.png" 
                                alt="0xbase.ai Logo" 
                                class="footer-logo"
                            />
                            <p class="footer-description">
                                { "0xbase.ai is a crypto-punk driven decentralized organization, focused on building AI tools on Basechain. We have extreme requirements and security obsessions, demanding all code to be written in Rust and completely open source." }
                            </p>
                        </div>
                        
                        <div class="footer-projects">
                            <a href="https://github.com/0xBaseAI/rings" target="_blank" rel="noopener noreferrer" class="project-link">
                                <img src="/imgs/rings.png" alt="Rings" class="project-logo" />
                                <div class="project-content">
                                    <div class="project-name">{ "Rings" }</div>
                                    <div class="project-desc">{ "P2P network with WebRTC & WASM" }</div>
                                    <div class="project-detail">{ "A decentralized peer-to-peer networking library built with Rust, featuring WebRTC for real-time communication and WebAssembly for cross-platform compatibility. Designed for high-performance, low-latency applications." }</div>
                                </div>
                            </a>
                            <div class="project-divider"></div>
                            <a href="https://github.com/0xBaseAI/castorix" target="_blank" rel="noopener noreferrer" class="project-link">
                                <img src="/imgs/castorix.png" alt="Castorix" class="project-logo" />
                                <div class="project-content">
                                    <div class="project-name">{ "Castorix" }</div>
                                    <div class="project-desc">{ "Farcaster protocol library" }</div>
                                    <div class="project-detail">{ "A comprehensive Rust implementation of the Farcaster protocol, providing secure and efficient tools for building decentralized social applications. Features include cryptographic signatures, message validation, and network synchronization." }</div>
                                </div>
                            </a>
                            <div class="project-divider"></div>
                            <a href="https://github.com/0xBaseAI/snaprag" target="_blank" rel="noopener noreferrer" class="project-link">
                                <img src="/imgs/snapRAG.png" alt="SnapRAG" class="project-logo" />
                                <div class="project-content">
                                    <div class="project-name">{ "SnapRAG" }</div>
                                    <div class="project-desc">{ "Farcaster data synchronization system" }</div>
                                    <div class="project-detail">{ "A high-performance data synchronization system designed specifically for Farcaster protocol data. It provides complete historical data synchronization, real-time monitoring, and profile management with PostgreSQL, optimized for RAG (Retrieval-Augmented Generation) applications." }</div>
                                </div>
                            </a>
                        </div>
                        
                        <p class="footer-text">
                            { "© 2025 0xbase.ai" }
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
