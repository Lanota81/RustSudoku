# RustSudoku
A simple Sudoku game based on Rust. 

## 提高点
- 基于 clap 的命令行应用程序，支持主要的功能子命令
- 采用随机算法生成数独
- 基于 confy, serde 的数据保存方法
- 良好封装的数独逻辑，高度可复用

## 细节问题
- 使用 `confy::get_configuration_file_path` 可查看本地的数据保存地址，遇到运行问题应删除 `Sudoku.toml` 所在文件夹
- Debug 模式下程序会输出随机算法重试次数和成功生成所用的运行时间，默认重试时间为 500ms
- 首次运行应使用 `cargo run help`(source code) 或 `./Sudoku help`(executable file) 查看帮助，并运行一次 `cargo run(./Sudoku) new` 以开始游戏
- 由于命令行程序特性，游戏是逐步保存的