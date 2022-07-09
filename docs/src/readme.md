# Introduction

figmaid is a simple web server that serves locally installed files to Figma.


## How does it work? (in a nutshell)

Figma allows local fonts through a background service they call FigmaAgent.
It works by establishing a web server with two jobs: serving locally installed fonts and a single index file with metadata about these fonts.

It comes with some limitations though, most importantly the platform support, or lack thereof.
FigmaAgent currently only supports Windows and macOS, but figmaid has minimal assumptions on where you run it.

Here's what Figma does when you load a document:
1. Request index file from localhost (that's your computer!)
2. Add fonts from the JSON response to selection
3. If user uses one of these fonts get the file


figmaid is a workhorse. It eats fonts and spits out their metadata simply by walking through your directories (platform defaults, and any you specify in [configuration](./configuration.md)), parses font tables and creates the index file. 


## In practice

1. Start the server and reload Figma, if open. Figma will now use fonts served by figmaid.
![Start the server and reload the page. Figma will now use fonts served by figmaid.](static/usage_1.png)
2. Individual font files are requested from the server.
![Individual font files are requested from the server](static/usage_2.png)