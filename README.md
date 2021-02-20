# 通过 cmake 混合构建 Rust & Cpp
**Rust** 提供了非常好用的 **FFI**, 可以方便我们将 **Rust** 代码跟 **C/Cpp** 之间互操. 在开始之前先弄个基本的种子例子, 我决定还是用 `SDL2` 来做演示场景. 后续有可能会尝试一下音视频之类的, `SDL2` 很适合拿来学习.

### 准备工作

先把 `SDL2` 装上, **macOS** 系统下就访问 [brew 中文](https://brew.sh/index_zh-cn) 官网, 可以找到 brew 的安装方法

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install.sh)"
```
如果已经安装了 brew, 那就直接安装 `sdl2`

```bash
brew install sdl2
```

然后再用 `cmake` 配置一个 **Cpp** 项目环境, 没有装 `cmake` 也可以用 `brew` 安装

```bash
brew install cmake
```

接下来就是
```bash
mkdir rust_client && cd rust_client
mkdir gui
touch gui/main.cpp
touch CMakeLists.txt
```

我们现在开始编辑 `CMakeLists.txt`

```cmake
cmake_minimum_required(VERSION 3.15)
set(SHORT_NAME cpp-with-rust)
project(${SHORT_NAME})
add_subdirectory(rs)
add_subdirectory(gui)
```
然后在 `gui` 目录下编辑 `CMakeLists.txt`, 具体 **SDL2** 目录以自己的环境为准

```cmake
set(CMAKE_CXX_STANDARD 14)

include(FindSDL2.cmake)
find_package(SDL2 REQUIRED)
include_directories(${SDL2_INCLUDE_DIRS})

set(SOURCE main.cpp)
add_executable(gui ${SOURCE})

get_target_property(CLIENT_DIR rs LOCATION)

target_link_libraries(gui ${SDL2_LIBRARIES})
target_link_libraries(gui ${CLIENT_DIR}/librs.dylib)

add_dependencies(gui rs)
```

这里的 `FindSDL2.cmake` 是一个 github 上人家的配置文件, 解决找不到 `SDL2` 的头文件的问题. 可以访问该地址下载 [SDL2Test](https://github.com/trenki2/SDL2Test).  
还有个问题 `get_target_property(RS_DIR rs LOCATION)` 是后面创建的 **Rust** 项目里 `CMakeLists.txt` 定义的, 现在先这样写.  
然后我们在 main.cpp 里用 `SDL2` 创建个窗口

```cpp
#include "SDL.h"

enum {
    WINDOW_WIDTH = 960,
    WINDOW_HEIGHT = 544,
    SCREEN_CENTER = SDL_WINDOWPOS_CENTERED_MASK,
};

int main(int argc, char **argv) {
    if (SDL_Init(SDL_INIT_VIDEO) < 0) return -1;
    SDL_Window *window = SDL_CreateWindow("Rust FFI Demo", SCREEN_CENTER, SCREEN_CENTER,
                                          WINDOW_WIDTH,
                                          WINDOW_HEIGHT, SDL_WINDOW_OPENGL | SDL_WINDOW_RESIZABLE);
    if (window == nullptr) return -1;

    SDL_Renderer *renderer = SDL_CreateRenderer(window, -1, 0);
    if (renderer == nullptr) return -1;

    SDL_Event event;
    while (true) {
        if (SDL_PollEvent(&event)) {
            if (SDL_QUIT == event.type) break;
            SDL_SetRenderDrawColor(renderer, 100, 0, 0, 255);
            SDL_RenderClear(renderer);
            SDL_RenderPresent(renderer);
        }
    }

    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
    SDL_Quit();

    return 0;
}
```

终于轮到 Rust 项目上场了, 直接在项目根目录下 `cargo` 走起

```bash
cargo new --lib rs
```
现在当前项目结构大概长这样

```bash
$ tree -L 3
.
├── CMakeLists.txt
├── build
├── gui
│   ├── CMakeLists.txt
│   ├── FindSDL2.cmake
│   └── main.cpp
└── rs
    ├── CMakeLists.txt
    ├── Cargo.lock
    ├── Cargo.toml
    ├── src
    │   └── lib.rs
    └── target
        └── debug
```
打开 `Cargo.toml` 文件编辑一下

```toml
[package]
name = "rs"
version = "0.1.0"
authors = ["Author <xxx@example.com>"]
edition = "2018"

[dependencies]

[lib]
crate-type = ["cdylib"]
```
我们主要关注点 `[lib]` 下的内容, 添加 `crate-type  ["cdylib"]`
这里意思是说创建 `C` 动态库, 其实也可以创建静态库, 具体参数是 `crate-type = ["cdylib", "staticlib"]` 其实也可以指定编译出来的库名字, 譬如指定为 `app`, 就是添加 `name = "app"`

重点来了, 创建 rs 目录下的 `CMakeLists.txt`, 添加如下内容
```cmake
if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build)
    set(TARGET_DIR "debug")
else ()
    set(CARGO_CMD cargo build --release)
    set(TARGET_DIR "release")
endif ()

set(RS_SO "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/librs.dylib")

add_custom_target(rs ALL
        COMMENT "Compiling rs module"
        COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD}
        COMMAND cp ${RS_SO} ${CMAKE_CURRENT_BINARY_DIR}
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})

set_target_properties(rs PROPERTIES LOCATION ${CMAKE_CURRENT_BINARY_DIR})

add_test(NAME rs_test
        COMMAND cargo test
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
```

现在为了保证项目干净整洁, 我们可以在项目根目录创建个 `build`, 然后再用 `cmake` 生成 make file

```bash
mkdir build && cd build
cmake ..
make
./gui/gui
```

看看效果, 我看到的效果是一个暗红色空白窗口. 好了现在整个项目算了配置完成了.

### Cpp 调用 Rust
通常我们学习当然是从 **Hello World** 开始的啦, 那就用控制台打印一下吧.
编辑 **Rust** src 下的 `lib.rs` 文件

```rust
#[no_mangle]
pub extern "C" fn hello_rust() {
    println!("Hello Rust!");
}
```
日常 `Hello Rust`, **no_mangle** 保证函数签名不被混淆, 这一步很重要, 不然 Cpp 调用的时候就会找不到对应的函数, 然后我们用 **Cpp** 调用一下.

```cpp
// ...
#include "rust.hpp"

// ...
int main(int argc, char **argv) {
    // ...
    bool quit = true;
    while (quit) {
        if (SDL_PollEvent(&event)) {
            if (SDL_QUIT == event.type) break;
            if (SDL_KEYDOWN == event.type) {
                switch (event.key.keysym.sym) {
                    case SDLK_ESCAPE:
                        quit = false;
                        break;
                    case SDLK_j:
                        hello_rust();
                        break;
                }
            }
            // ...
        }
    }
    // ...
    return 0;
}
```

肯定注意到有个 `#include "rust.hpp"`, 原来是把 `hello_rust` 函数 `extern` 出来

```hpp
// rust.hpp
extern "C" {
    void hello_rust();
};
```

然后在 **SDL** 主线程事件循环中通过键盘字母 `j` 触发调用, 也稍微改了点 退出的逻辑. 最后别忘了在 gui 目录下 CMakeLists.txt 中修改一下 **SOURCE**

```cmake
# ...
set(SOURCE rust.hpp main.cpp)
# ...
```
最后重复构建操作
```bash
cd build
cmake ..
make
./gui/gui
```
执行一下看看效果, 只要按了字母 `j` 就会在控制台打印 **Hello Rust!**  
看样子简单的 **Cpp** 调用 **Rust** 完成了.

----
项目地址

https://github.com/limitLiu/cpp-with-rust.git
