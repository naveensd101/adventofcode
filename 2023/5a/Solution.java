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

        File passFile1 = new File("IP/pass1");
        File passFile2 = new File("IP/pass2");
        File passFile3 = new File("IP/pass3");
        File passFile4 = new File("IP/pass4");
        File passFile5 = new File("IP/pass5");
        File passFile6 = new File("IP/pass6");
        File passFile7 = new File("IP/pass7");

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
        List<Long> seeds = Arrays.asList(
                487758422L,
                524336848L,
                2531594804L,
                27107767L,
                1343486056L,
                124327551L,
                1117929819L,
                93097070L,
                3305050822L,
                442320425L,
                2324984130L,
                87604424L,
                4216329536L,
                45038934L,
                1482842780L,
                224610898L,
                115202033L,
                371332058L,
                2845474954L,
                192579859L

        );
        Long answer = Long.MAX_VALUE;
        for (int i = 0; i < seeds.size(); i++) {
            Long pass0Answer = seeds.get(i);
            Long pass1Answer = processor(pass0Answer, pass1);
            Long pass2Answer = processor(pass1Answer, pass2);
            Long pass3Answer = processor(pass2Answer, pass3);
            Long pass4Answer = processor(pass3Answer, pass4);
            Long pass5Answer = processor(pass4Answer, pass5);
            Long pass6Answer = processor(pass5Answer, pass6);
            Long pass7Answer = processor(pass6Answer, pass7);
            answer = Math.min(answer, pass7Answer);
        }
        System.out.println(answer);
    }
}
