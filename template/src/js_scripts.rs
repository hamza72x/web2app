pub const INIT: &str = r#"
    // load darkreader.js
    (function() {
        window.onload = function() {
            console.log('Window is loaded');
            console.log('Loading DarkReader');
            var script = document.createElement('script');
            script.src = 'https://cdn.jsdelivr.net/npm/darkreader@4.9.58/darkreader.min.js';
            script.onload = function() {
                console.log('DarkReader is loaded');
            };
            document.head.appendChild(script);
        };
    })();
"#;
