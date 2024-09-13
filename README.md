# HKX Conversion Tool

HKX Conversion Tool is a GUI wrapper for hkxc.exe for converting Havok files between different formats for Skyrim modding. 

I initially started building a wrapper for hkxcmd, but since that doesn't handle amd64 hkx files, I was also integrating hkxconv to convert those into xmls first and pushing those xmls through hkxcmd, and then I got upset that the solution was messy and started to look at their source code to try and build a modern version that handles win32, amd64 and xml format conversions between each other. Thankfully I didn't get too far into reinventing the wheel before I found out someone built this already, so I just repurposed the GUI tool for their tool since It Just Works™.

Working with havok files is really a pain in the ass, since Blender's hkx plugin can't import Skyrim SE/amd64 hkx formats, they have to all be converted to win32 Oldrim format first. A lot of the existing guides will tell you to find and install Havok Content Tools and use hctStandAloneFilterManager.exe. That pissed me off every time I had to edit an existing SE animation because you have to do one animation at a time and the whole process takes a lot of clicks every time.

## Features

- Convert HKX files to XML format
- Convert XML files to HKX format for Skyrim LE and SE
- Batch conversion support
- User-friendlier GUI interface
- Specify output folder and suffix options

## Installation

1. Download the latest release.
2. Extract the zip file to your desired location.
3. Run `hkxtools.exe` file.

## Usage

1. Launch the application.
2. Click "Browse Input Files" to select the HKX or XML files you want to convert.
3. (Optional) Set the output folder by clicking "Browse" next to "Output Folder".
4. (Optional) Enter a suffix for the output files.
5. Select the desired output format (XML, Skyrim LE, or Skyrim SE).
6. Click "Run Conversion" to start the conversion process.

## License

This project is licensed under the MIT License - see below for details:

MIT License

Copyright (c) 2023

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

## Credits

- hkxc.exe: This project uses the CLI hkxc.exe from [serde-hkx](https://www.nexusmods.com/skyrimspecialedition/mods/126214/) by SARDONYX.
