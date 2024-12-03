import java.io.BufferedReader;
import java.io.File;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Scanner;

public class Solution {
    static List<List<Long>> pass1;
    static List<List<Long>> pass2;
    static List<List<Long>> pass3;
    static List<List<Long>> pass4;
    static List<List<Long>> pass5;
    static List<List<Long>> pass6;
    static List<List<Long>> pass7;
    private static Long processor(Long start, List<List<Long>> pass) {
        Long answer = start;
        for (int i = 0; i < pass.size(); i++) {
            if(pass.get(i).get(1) <= start && start <= pass.get(i).get(1) + pass.get(i).get(2) - 1) {
                answer = pass.get(i).get(0) + start - pass.get(i).get(1);
            }
        }
        return answer;
    }
    private static void InputProcessing() throws IOException {
        pass1 = new ArrayList<>();
        pass2 = new ArrayList<>();
        pass3 = new ArrayList<>();
        pass4 = new ArrayList<>();
        pass5 = new ArrayList<>();
        pass6 = new ArrayList<>();
        pass7 = new ArrayList<>();

        File passFile1 = new File("testingIP/pass1");
        File passFile2 = new File("testingIP/pass2");
        File passFile3 = new File("testingIP/pass3");
        File passFile4 = new File("testingIP/pass4");
        File passFile5 = new File("testingIP/pass5");
        File passFile6 = new File("testingIP/pass6");
        File passFile7 = new File("testingIP/pass7");

        BufferedReader bufferedReader1 = new BufferedReader(new FileReader(passFile1));
        BufferedReader bufferedReader2 = new BufferedReader(new FileReader(passFile2));
        BufferedReader bufferedReader3 = new BufferedReader(new FileReader(passFile3));
        BufferedReader bufferedReader4 = new BufferedReader(new FileReader(passFile4));
        BufferedReader bufferedReader5 = new BufferedReader(new FileReader(passFile5));
        BufferedReader bufferedReader6 = new BufferedReader(new FileReader(passFile6));
        BufferedReader bufferedReader7 = new BufferedReader(new FileReader(passFile7));

        String line;
        while( (line = bufferedReader1.readLine()) != null) {
            Scanner scanner = new Scanner(line);
            List<Long> thisOne = new ArrayList<>();
            while (scanner.hasNextLong()) {
                thisOne.add(scanner.nextLong());
            }
            pass1.add(thisOne);
        }
        while( (line = bufferedReader2.readLine()) != null) {
            Scanner scanner = new Scanner(line);
            List<Long> thisOne = new ArrayList<>();
            while (scanner.hasNextLong()) {
                thisOne.add(scanner.nextLong());
            }
            pass2.add(thisOne);
        }
        while( (line = bufferedReader3.readLine()) != null) {
            Scanner scanner = new Scanner(line);
            List<Long> thisOne = new ArrayList<>();
            while (scanner.hasNextLong()) {
                thisOne.add(scanner.nextLong());
            }
            pass3.add(thisOne);
        }
        while( (line = bufferedReader4.readLine()) != null) {
            Scanner scanner = new Scanner(line);
            List<Long> thisOne = new ArrayList<>();
            while (scanner.hasNextLong()) {
                thisOne.add(scanner.nextLong());
            }
            pass4.add(thisOne);
        }
        while( (line = bufferedReader5.readLine()) != null) {
            Scanner scanner = new Scanner(line);
            List<Long> thisOne = new ArrayList<>();
            while (scanner.hasNextLong()) {
                thisOne.add(scanner.nextLong());
            }
            pass5.add(thisOne);
        }
        while( (line = bufferedReader6.readLine()) != null) {
            Scanner scanner = new Scanner(line);
            List<Long> thisOne = new ArrayList<>();
            while (scanner.hasNextLong()) {
                thisOne.add(scanner.nextLong());
            }
            pass6.add(thisOne);
        }
        while( (line = bufferedReader7.readLine()) != null) {
            Scanner scanner = new Scanner(line);
            List<Long> thisOne = new ArrayList<>();
            while (scanner.hasNextLong()) {
                thisOne.add(scanner.nextLong());
            }
            pass7.add(thisOne);
        }
    }
    public static void main(String[] argc) {
        System.out.println("HI");
        try {
            InputProcessing();
        }
        catch (IOException exception) {
            System.out.println("File not found");
        }
        List<Pair<Long, Long>> seeds = Arrays.asList(
                new Pair<Long, Long>(79L,14L),
                new Pair<Long, Long>(55L,13L)
        );
        System.out.println(seeds.toString());
    }
}
