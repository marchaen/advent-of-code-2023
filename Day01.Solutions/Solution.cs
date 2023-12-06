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
}
