class sudokuSolve  {
    int WIDTH = 9;
    int HEIGHT = 9;
    int SCREEN_WIDTH = HEIGHT * 3 + 4;
    int ROUND_LIMIT = 1000;

    int start_world[][] = new int[WIDTH][HEIGHT];
    boolean possibilities[][][] = new boolean[WIDTH][HEIGHT][WIDTH + 1]; //width + 1 so we don't have to use offsets 

    public static void main(String[] args) {
        System.out.println("Hello, World!"); 
    }
}
