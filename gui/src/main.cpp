#include "SDL.h"
#include "rust.h"

typedef SDL_Window *pWindow;
enum {
    WINDOW_WIDTH = 960,
    WINDOW_HEIGHT = 544,
    SCREEN_CENTER = SDL_WINDOWPOS_CENTERED_MASK,
};

int main(int argc, char **argv) {
  if (SDL_Init(SDL_INIT_VIDEO) < 0) return -1;
  pWindow window = SDL_CreateWindow("Rust FFI Demo", SCREEN_CENTER, SCREEN_CENTER,
                                    WINDOW_WIDTH,
                                    WINDOW_HEIGHT, SDL_WINDOW_OPENGL | SDL_WINDOW_RESIZABLE);
  if (window == nullptr) return -1;

  SDL_Renderer *renderer = SDL_CreateRenderer(window, -1, 0);
  if (renderer == nullptr) return -1;

  SDL_Event event;
  bool quit = true;

  while (quit) {
    printf("%f\n", get_time());
    if (SDL_PollEvent(&event)) {
      if (SDL_QUIT == event.type) break;
      if (SDL_KEYDOWN == event.type) {
        switch (event.key.keysym.sym) {
          case SDLK_ESCAPE:
            quit = false;
            break;
          case SDLK_j:
            printf("%lf\n", my_sqrt(2));
            printf("%lf\n", my_cbrt(27));
            break;
        }
      }
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
