# Living CV

Living CV is an experimental project to combine the immediate mode GUI library egui with the typsesetting system typst to enhance
a static cv with dynamic information.

## Features
- Render a typst file as an image and use it as a background for an eframe.
- Use typst as a library to analyze the document and find the interesting text blocks.

## Usage
This project takes a typst file, renders it in an eframe and uses egui to enhance the cv with additional information that doesn't fit on the cv itself.

# Credits
- [egui](https://github.com/emilk/egui) for the great immediate mode GUI
- [eframe_template](https://github.com/emilk/eframe_template) as a starting point of this project
- [typst-as-a-library](https://github.com/tfachmann/typst-as-library) for the help of how to use typst as a library
