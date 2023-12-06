namespace Day01.Solutions;

public class Solution
{
    /// <summary>
    ///     Solution for the first part of day 01 of AoC. The following problem
    ///     description was given.
    ///
    ///     <para>
    ///         The newly-improved calibration document consists of lines of
    ///         text; each line originally contained a specific calibration
    ///         value that these Elves now need to recover. On each line, the
    ///         calibration value can be found by combining the first digit and
    ///         the last digit (in that order) to form a single two-digit
    ///         number.
    ///
    ///         For example:
    ///
    ///         1abc2
    ///         pqr3stu8vwx
    ///         a1b2c3d4e5f
    ///         treb7uchet
    ///
    ///         In this example, the calibration values of these four lines are
    ///         12, 38, 15, and 77. Adding these together produces 142.
    ///     </para>
    /// </summary>
    public static int PartOne(string input)
    {
        int sum = 0;

        var lines = input.Split(
            '\n',
            StringSplitOptions.TrimEntries | StringSplitOptions.RemoveEmptyEntries
        );

        foreach (var line in lines)
        {
            char? left = null;
            char? right = null;

            foreach (var character in line.ToCharArray())
            {
                if (char.IsDigit(character))
                {
                    if (left != null)
                    {
                        right = character;
                    }
                    else
                    {
                        left = character;
                    }
                }
            }

            left ??= '0';
            right ??= left;
            sum += int.Parse($"{left}{right}");
        }

        return sum;
    }

    /// <summary>
    ///     Solution for the second part of day 01 of AoC. The following problem
    ///     description was given.
    ///     <para>
    ///         Your calculation isn't quite right. It looks like some of the
    ///         digits are actually spelled out with letters: one, two, three,
    ///         four, five, six, seven, eight, and nine also count as valid
    ///         "digits".
    ///
    ///         Equipped with this new information, you now need to find the
    ///         real first and last digit on each line. For example:
    ///
    ///         two1nine
    ///         eightwothree
    ///         abcone2threexyz
    ///         xtwone3four
    ///         4nineeightseven2
    ///         zoneight234
    ///         7pqrstsixteen
    ///
    ///         In this example, the calibration values are 29, 83, 13, 24, 42,
    ///         14, and 76. Adding these together produces 281.
    ///     </para>
    /// </summary
    public static int PartTwo(string input)
    {
        int sum = 0;

        var lines = input.Split(
            '\n',
            StringSplitOptions.TrimEntries | StringSplitOptions.RemoveEmptyEntries
        );

        foreach (var line in lines)
        {
            var nameOfNumber = (int n) =>
                n switch
                {
                    1 => "one",
                    2 => "two",
                    3 => "three",
                    4 => "four",
                    5 => "five",
                    6 => "six",
                    7 => "seven",
                    8 => "eight",
                    9 => "nine",
                    _ => ""
                };

            int? left = null;
            int? right = null;

            for (int index = 0; index < line.Length; index++)
            {
                if (char.IsDigit(line[index]))
                {
                    if (left != null)
                        right = int.Parse(line[index].ToString());
                    else
                        left = int.Parse(line[index].ToString());

                    continue;
                }

                for (int number = 1; number < 10; number++)
                {
                    if (!line[index..].StartsWith(nameOfNumber(number)))
                        continue;

                    if (left != null)
                        right = number;
                    else
                        left = number;

                    break;
                }
            }

            left ??= 0;
            right ??= left;
            sum += int.Parse($"{left}{right}");
        }

        return sum;
    }
}
