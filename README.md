# FindBiggestFile.rs

[FindBiggestFile](https://github.com/Angr1st/FindBiggestFile) but written in [rust](https://www.rust-lang.org/). Using a config file, supplied via command line arguments, you can search for the biggest files with a certain name or for the biggest file inside a folder with a given extension.

## Build

Build it using ```cargo build --release```.

## Create Example Config

Running it with ```./find_biggest_file.exe --init``` generates an example config file for you to fill out. You can also find the example config inside this [repository](/example/Example_Config.json).

## Supplying a Config

Running it with ```./find_biggest_file.exe --configfilepath Example_Config.json``` or ```./find_biggest_file.exe -c Example_Config.json```  executes the search for your files.

## Finding the biggest file of a certain type in a certain folder

Using the "BiggestFileInFolder" you can search for the biggest file with a certain file type in a given folder.
