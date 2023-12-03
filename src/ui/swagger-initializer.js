window.onload = async function() {
    const urls = await (await fetch("urls")).json();
    window.ui = SwaggerUIBundle({
        urls,
        dom_id: '#swagger-ui',
        supportedSubmitMethods: [],
        deepLinking: true,
        presets: [
            SwaggerUIBundle.presets.apis,
            SwaggerUIStandalonePreset
        ],
        plugins: [
            SwaggerUIBundle.plugins.DownloadUrl,
            InfoCollapsePlugin
        ],
        layout: "StandaloneLayout"
    });
};

const InfoCollapsePlugin = (system) => {
    const React = system.React;
    return {
        wrapComponents: {
            info: (Original) => (props) => React.createElement(
                "details",
                {
                    open: true,
                    id: "collapsible-info"
                },
                React.createElement(
                    "summary",
                    {
                        style: {
                            cursor: "pointer",
                            backgroundColor: "#E0E0E0",
                            "text-align": "center",
                            "font-style": "italic"
                        }
                    },
                    "Info"
                ),
                React.createElement(
                    Original,
                    props
                )
            )
        }
    }
}
