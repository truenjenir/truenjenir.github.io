use leptos::*;
use leptos::prelude::*;

fn main() {
    // Mount the website to the body of the HTML document
    mount_to_body(|| {
        view! {
            <div class="container">
                <header>
                    <h1 class="title">Welcome to My Awesome Website</h1>
                    <nav>
                        <ul class="nav-list">
                            <li><a href="#about">About</a></li>
                            <li><a href="#services">Services</a></li>
                            <li><a href="#contact">Contact</a></li>
                        </ul>
                    </nav>
                </header>
                <main>
                    <section id="about" class="section">
                        <h2>About Us</h2>
                        <p>
                            We are a creative and innovative team dedicated to building
                            amazing web experiences using Rust and the leptos library.
                        </p>
                    </section>
                    <section id="services" class="section">
                        <h2>Our Services</h2>
                        <ul class="service-list">
                            <li>Web Design</li>
                            <li>Web Development</li>
                            <li>Mobile App Development</li>
                        </ul>
                    </section>
                    <section id="contact" class="section">
                        <h2>Contact Us</h2>
                        <p>
                            Feel free to reach out to us at
                            <a href="mailto:contact@example.com">contact@example.com</a>.
                        </p>
                    </section>
                </main>
                <footer>
                    <p>&copy; 2023 My Awesome Website</p>
                </footer>
            </div>
        }
    });
}
