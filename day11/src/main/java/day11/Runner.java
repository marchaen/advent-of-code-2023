package day11;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;

public class Runner {
    public static void main(String[] args) {
        try {
            var input = Files.readString(Path.of("..", "input", "Day11.txt"));
            System.out.println(Solution.partOne(input));
        } catch (IOException ignored) {
            System.err.println("Couldn't read puzzle input. Run from project root dir!");
        }
    }
}
