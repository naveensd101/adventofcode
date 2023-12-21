import java.io.BufferedReader;
import java.io.ByteArrayInputStream;
import java.io.File;
import java.io.FileReader;
import java.io.InputStream;
import java.util.ArrayList;
import java.util.List;
import java.util.Locale;
import java.util.Scanner;

public class Solution {
    /**
     * Input: Single line of the game
     * <br/>
     * Output: A list that contains list of sub-games in the order [red, green, blue]
     * <br/>
     * Input: Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
     * <br/>
     * Output: [[1], [4, 0, 3], [1, 2, 6], [0, 2, 0]]
     */
    private static List<List<Integer>> processInput(String line) {
        List<List<Integer>> result = new ArrayList<>();
        String splitedWithColon = line.split(":")[0];

        // Extracting game number
        String gameNumberStr = splitedWithColon.substring(5, splitedWithColon.length());
        Integer gameNumber = Integer.parseInt(gameNumberStr);
        result.add(List.of(gameNumber));

        // Extracting first game
        String[] games = line.split(":")[1].split(";");
        for (int i = 0; i < games.length; i++) {
            Integer red = 0, green = 0, blue = 0;
            String[] game = games[i].split(",");
            for (int j = 0; j < game.length; j++) {
                Scanner scanner = new Scanner(game[j]);
                int number = scanner.nextInt();
                String color = scanner.nextLine();
                switch (color) {
                    case " red": red += number; break;
                    case " blue": blue += number; break;
                    case " green": green += number; break;
                }
            }
            result.add(List.of(red, green, blue));
        }

        return result;
    }
    public static void main(String[] argc) throws Exception {
        File inputFile = new File("ip.final");
        BufferedReader bufferedReader = new BufferedReader(new FileReader(inputFile));
        String line;
        Integer answer = 0;
        while ( (line = bufferedReader.readLine()) != null) {
            List<List<Integer>> inputList = processInput(line);
            System.out.println(line);
            System.out.println(inputList.toString());
            // 12 red cubes, 13 green cubes, and 14 blue cubes
            // [12, 13, 14]
            boolean possible = true;
            for (int i = 1; i < inputList.size(); i++) {
                System.out.println(inputList.get(i).toString());
                if( inputList.get(i).get(0) > 12
                        || inputList.get(i).get(1) > 13
                        || inputList.get(i).get(2) > 14 ) {
                    possible = false;
                    break;
                }
            }
            if(possible) {
                System.out.println("possible id = " + inputList.get(0).get(0));
                answer = answer + inputList.get(0).get(0);
            }
        }
        System.out.println("ans = " + answer);
    }
}
