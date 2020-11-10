#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define WIDTH 9
#define HEIGHT 9
#define SCREEN_WIDTH WIDTH *3 + 4
#define ROUND_LIMIT 1000

int start_world[WIDTH][HEIGHT] = {
   {0,0,0,0,0,0,0,0,0},
   {0,0,0,0,0,0,0,0,0},
   {0,0,0,0,0,0,0,0,0},
   {0,0,0,0,0,0,0,0,0},
   {0,0,0,0,0,0,0,0,0},
   {0,0,0,0,0,0,0,0,0},
   {0,0,0,0,0,0,0,0,0},
   {0,0,0,0,0,0,0,0,0},
   {0,0,0,0,0,0,0,0,0}
};

int possibilities[WIDTH][HEIGHT][WIDTH + 1]; 

int parse(char * arg) {
   int i, x, y = 0;
   if (strlen(arg) != 81) {
      printf("%i\n" , strlen(arg));
      return -1;
   }

   for (i = 0 ; i < strlen(arg) ; i++) {
      x = i % 9;
      y = i / 9;

      start_world[x][y] = (int)arg[i] - 48; //(char 0) == 48
   }

   return 1;
}

void printBoard() {
   int x,y,i = 0;

   for (y = 0 ; y < 9 ; y++) {
      if (y % 3 == 0) {
         if (y > 0 ) printf("\n");
         for (i = 0 ; i < SCREEN_WIDTH ; i++) printf("-");
      }
      printf("\n|"); // newline

      for (x = 0 ; x < 9 ; x++) {
         if (x % 3 == 0 && x > 0) {
            printf("|");
         }
         printf(" %i ", start_world[x][y]);
         if (x == 8) printf("|");
      }
   }
   printf("\n");
   for (i = 0 ; i < SCREEN_WIDTH ; i++) printf("-");
   printf("\n");
}

int exists_in_row(int row, int n) {
   int i;

   for (i = 0 ; i < 9 ; i++ ) {
      if(start_world[row][i] == n) return 1;
   }

   return 0;
}

int exists_in_col(int col, int n) {
   int i;

   for (i = 0 ; i < 9 ; i++ ) {
      if(start_world[i][col] == n) return 1;
   }

   return 0;
}

int exists_in_box(int nx, int ny, int n) {
   // 0,1,2 - 3,4,5 - 6,7,8 
   int cx = nx / 3;
   int cy = ny / 3;
   int x,y;


   for (x = 0 ; x < 3 ; x++ ) {
      for ( y = 0 ; y < 3 ; y++ ) {
         if (start_world[x + (cx * 3)][y + (cy * 3)] == n) return 1;
      }
   }

   return 0;
}

int possible_num(int x, int y, int n) {
   if(!exists_in_col(y, n) &&
         !exists_in_row(x, n) &&
         !exists_in_box(x, y, n)) {
      return 1;
   } 

   return 0;
}

void calculate_possibilities(int x, int y) {
   int i;

   for (i = 1 ; i <= 9 ; i++) {
      if(possible_num(x, y, i)) {
         possibilities[x][y][i] = i;
      } else {
         possibilities[x][y][i] = 0;
      }
   }
}

int is_only_possibility_box(int x, int y, int i) {
   int bx = x / 3;
   int by = y / 3;
   int sx,sy;
   int posscount = 0;

   for (sx = 0 ; sx < 3 ; sx++) {
      for (sy = 0 ; sy < 3 ; sy++) {
         if(possibilities[(bx * 3) + sx][(by * 3) + sy][i] > 0) {
            if(start_world[(bx * 3) + sx][(by * 3) + sy] == 0) {
               posscount += 1;
            }
         }
      }
   }

   if(posscount == 1) return 1;
   return 0;
}

int is_only_possibility_row(int x, int y, int i) {
   //x = row;
   int n;
   int posscount = 0;

   for (n = 0 ; n < 9 ; n++) {
      if (possibilities[x][n][i] > 0) {
         if (start_world[x][n] == 0) {
            posscount += 1;
         }
      }
   }

   if(posscount == 1) return 1;
   return 0;
}

int is_only_possibility_col(int x, int y, int i) {
   int n;
   int posscount = 0;

   for (n = 0 ; n < 9 ; n++) {
      if (possibilities[n][y][i] > 0) {
         if (start_world[n][y] == 0) {
            posscount += 1;
         }
      }
   }

   if(posscount == 1) return 1;
   return 0;
}

int solve() {
   int x, y, i;
   int round = 0;
   int nonempty = 1;
   int placed = 0;

   while(round < ROUND_LIMIT) {
   //   printf("Round : %d\n", round);
      round++;
      nonempty = 0;
      placed = 0;
      //populate possibilities
      for (x = 0 ; x < 9 ; x++) {
         for (y = 0 ; y < 9 ; y++) {
            if (start_world[x][y] == 0) {
               nonempty = 1;
               calculate_possibilities(x, y);
            }
         }
      }
      //resolve
      for (x = 0 ; x < 9 ; x++) {
         for (y = 0 ; y < 9 ; y++) {
            for (i = 1 ; i <= 9 ; i++) {
               if(start_world[x][y] == 0) {
                  nonempty = 1;
                  if (possibilities[x][y][i] > 0 && !placed) {
                     if(is_only_possibility_box(x,y,i) || is_only_possibility_row(x,y,i) || is_only_possibility_col(x,y,i)) {
                        start_world[x][y] = i;
                        placed = 1;
                     }
                  }
               }
            }
         }
      }

      if(!nonempty) break;
   }
      
   if(round >= ROUND_LIMIT) {
      printf("Reached round limit\n");
   }


   return 1;
}

int main(int argc, char * argv[]) {
   // printf() displays the string inside quotation
   if (argc != 2 ) {
      printf("Incorrect amount of arguments, format is ./sudoku-solve [sudoku]\n");
      return 0;
   }

   int res = parse(argv[1]);

   if (res != 1) {
      printf("Incorrect input format, please use a string of 82 integers\n");
      return 0;
   }

   printBoard();
   printf("Commencing the solve.....\n");
   res = solve();
   if (res) {
      printf("Puzzle solved : \n");
      printBoard();
      return 0;
   } else {
      printf("Puzzle cannot be solved..\n");
      return -1;
   }
   return 0;
}
