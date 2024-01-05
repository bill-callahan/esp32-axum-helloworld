
use axum::response::Html;

pub(crate) async fn helloworld() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

pub(crate) async fn clock_html() -> Html<String> {
    let raw = Html(format!(
        r#"<!doctype html>
        <html>
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>esp32 clock app webserver</title>
            <style>
                body {{
                    font-family: Arial, sans-serif;
                    text-align: center;
                    margin: 0;
                    padding: 0;
                    display: flex;
                    flex-direction: row;
                    height: 100vh;
                }}

                #left-section {{
                    flex: 1;
                    background-color: red;
                    color: white;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                }}

                #middle-section {{
                    flex: 1;
                    background-color: white;
                    color: black;     
                    align-items: center;
                    justify-content: center;
                }}

                #right-section {{
                    flex: 1;
                    background-color: blue;
                    color: white;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                }}

                #clock {{
                    font-size: 3rem;
                }}
                #mw {{
                    flex: 1;
                    background-color: white;
                    color: blue;     
                    align-items: center;
                    justify-content: center;
                }}
            </style>
        </head>
        <body>
            <div id="left-section">
                <h1></h1>
            </div>
            <div id="middle-section">
                <h1>Clock App via Axum Server on ESP32</h1>
                <div id="clock"></div>
                <div id="mw"><h1>By Bill Callahan</h1></div>
            </div>
            <div id="right-section">
                <h1></h1>
            </div>

            <script>
                function updateClock() {{
                    const clockElement = document.getElementById('clock');
                    const now = new Date();
                    const hours = now.getHours().toString().padStart(2, '0');
                    const minutes = now.getMinutes().toString().padStart(2, '0');
                    const seconds = now.getSeconds().toString().padStart(2, '0');

                    const timeString = `${{hours}}:${{minutes}}:${{seconds}}`;
                    clockElement.textContent = timeString;
                }}

                // Update the clock every second
                setInterval(updateClock, 1000);

                // Initial call to set the clock
                updateClock();
            </script>
        </body>
        </html>
        "#));
    raw
    
}