```mermaid
graph TD
    subgraph TextEditor
        subgraph Platform
            C#
            subgraph PluginSystem
                g[git]
                etc.
            end
            subgraph UI
                Blazor
            end
        end

        subgraph Engine
            subgraph APILayer
                eLang[Rust]
                f[File IO]
                e[Editor Actions]
                l[LSP]
            end
        end
    end

    Platform --> APILayer
    APILayer --> Platform
    PluginSystem -->|API Calls| APILayer
    PluginSystem -->|Provides UI and Functions|UI
```