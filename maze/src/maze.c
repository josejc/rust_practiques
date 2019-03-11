/* Robert Nystrom @munificentbob for Ginny 2008-2019 */
#include <time.h>
#include <stdio.h>
#include <stdlib.h>

const int HEIGHT = 40;
const int WIDTH  = 80;
int map[40][80];

const int PLAYER   = '@';
const int TREASURE = '$';
const int ROCK     = ' ';
const int CORNER   = '!';
const int WALL     = '#';
const int FLOOR    = '.';
const int DOOR1    = '+';
const int DOOR2    = '\'';

void
cave(int start)
{
  int width = (rand() % 10) + 5;
  int height = (rand() % 6) + 3;
  int left = (rand() % (WIDTH - width - 2)) + 1;
  int top = (rand() % (HEIGHT - height - 2)) + 1;

  for (int y = top - 1; y < top + height + 2; y++)
    for (int x = left - 1; x < left + width + 2; x++)
      if (map[y][x] == FLOOR)
        return;

  int doors = 0;
  int door_x, door_y;

  if (!start) {
    for (int y = top - 1; y < top + height + 2; y++)
      for (int x = left - 1; x < left + width + 2; x++) {
        int s = x < left || x > left + width;
        int t = y < top || y > top + height;
        if (s ^ t && map[y][x] == WALL) {
          doors++;
          if (rand() % doors == 0)
            door_x = x, door_y = y;
        }
      }

    if (doors == 0)
      return;
  }

  for (int y = top - 1; y < top + height + 2; y++)
    for (int x = left - 1; x < left + width + 2; x++) {
      int s = x < left || x > left + width;
      int t = y < top || y > top + height;
      map[y][x] = s && t ? CORNER : s ^ t ? WALL : FLOOR;
    }

  if (doors > 0)
    map[door_y][door_x] = (rand() % 2) ? DOOR2 : DOOR1;

  for (int j = 0; j < (start ? 1 : (rand() % 6) + 1); j++)
    map[(rand() % height) + top][(rand() % width) + left] =
      start ? PLAYER :
      (rand() % 4) == 0 ? TREASURE : 'A' + (rand() % 62);
}

int
main(int argc, const char* argv[])
{
  srand((int)time(NULL));
  for (int y = 0; y < HEIGHT; y++)
    for (int x = 0; x < WIDTH; x++)
      map[y][x] = ROCK;

  for (int j = 0; j < 1000; j++)
    cave(j == 0);

  for (int y = 0; y < HEIGHT; y++)
    for (int x = 0; x < WIDTH; x++) {
      int c = map[y][x];
      putchar(c == CORNER ? WALL : c);
      if (x == WIDTH - 1)
        printf("\n");
    }
  return 0;
}
