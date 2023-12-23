import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collection;
import java.util.Collections;
import java.util.List;

public class Solution {
    static List<String> Engine = new ArrayList<>();
    static List<List<Boolean>> Layout = new ArrayList<>();

    static boolean isCoordinateValid(int i, int j) {
        if(!(i >= 0 && i < Engine.size())) return false;
        if(!(j >= 0 && j < Engine.get(i).length())) return false;
        return true;
    }
    static boolean isDigit(int i, int j) {
        if(isCoordinateValid(i, j) && Engine.get(i).charAt(j) >= '0' && Engine.get(i).charAt(j) <= '9') return true;
        return false;
    }
    static void setLayout(List<Integer> ij) {
        if (isCoordinateValid(ij.get(0), ij.get(1))) {
            Layout.get(ij.get(0)).set(ij.get(1), true);
        }
        return;
    }
    /**
     * Input: (i, j) of grid in Engine
     * <br/>
     * Output: Return List[I,J] if There is number to the left of (i,j) starting from (i,j)
     * <br/>
     * If No number return List[-1,-1]
     */
    static List<Integer> checkLeft(int i, int j) {
        if(!isCoordinateValid(i,j)) return List.of(-1,-1);
        Boolean gotIn = false;
        while(isCoordinateValid(i,j) && isDigit(i,j)) {
            gotIn = true;
            j--;
        }
        if(gotIn) j++;
        if(isCoordinateValid(i,j) && isDigit(i,j)) return List.of(i,j);
        return List.of(-1,-1);
    }

    /**
     *
     * Input: (i,j) coordinates
     * <br/>
     * Output: Get the Integer starting from (i, j) In Engine. We will not check if (i,j) actually has a integer or not.
     */
    static Integer getRight(int i, int j) {
        String numberString = "";
        while(isCoordinateValid(i,j) && isDigit(i,j)) {
            numberString += Engine.get(i).charAt(j);
            j++;
        }
        return Integer.parseInt(numberString);
    }


    public static void main(String[] argc) throws Exception {
        File file = new File("ip.final");
        BufferedReader bufferedReader = new BufferedReader(new FileReader(file));

        String line;
        while ((line = bufferedReader.readLine()) != null) {
            Engine.add(line);
            Layout.add(new ArrayList<Boolean>(Collections.nCopies(line.length(), false)));
        }

        for (int i = 0; i < Engine.size(); i++) {
            for (int j = 0; j < Engine.get(i).length(); j++) {
                // Checking if the character at (i, j) is a Special char
                if (Engine.get(i).charAt(j) == '.' || isDigit(i,j)) {
                    continue;
                }

                setLayout(checkLeft(i-1, j+1));
                setLayout(checkLeft(i-1, j));
                setLayout(checkLeft(i-1, j-1));
                setLayout(checkLeft(i, j+1));
                setLayout(checkLeft(i, j-1));
                setLayout(checkLeft(i+1, j+1));
                setLayout(checkLeft(i+1, j));
                setLayout(checkLeft(i+1, j-1));
            }
        }
        Integer answer = 0;
        for (int i = 0; i < Engine.size(); i++) {
            for (int j = 0; j < Engine.get(i).length(); j++) {
                if (Layout.get(i).get(j)) {
                    answer += getRight(i,j);
                }
            }
        }
        System.out.println("answer = " + answer);
    }
}
