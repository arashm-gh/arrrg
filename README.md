# ARRRG

## Introduction
`arrg` is a command line tool written in Rust that uses apibay.org to fetch magnet links of the top $n$ number of torrents!  

## Usage

The format for running a command in arrrg is as follows:
```bash
arrrg <query> <limit>
```

Example 1:  
```bash
arrrg "Ubuntu" 10
```
This will return the top 10 magnet links in table format for the search term `Ubunutu` (case insensitive.)

Example 2:
```bash
arrrg "how i met your mother" 100
```
This will return the top 100 magnet links in table format for the search term `How I met your Mother` (case insensitive.)

## Installation
Run the following command in a terminal:
```bash
curl -fsSL https://raw.githubusercontent.com/arashm-gh/arrrg/main/install.sh | bash
```

## License
Please view LICENSE.md
