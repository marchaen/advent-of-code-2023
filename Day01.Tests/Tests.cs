using Day01.Solutions;

namespace Day01.Tests;

[TestClass]
public class Tests
{

    [TestMethod]
    public void TestPartOne()
    {
        var input = """
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        """;

        Assert.AreEqual(142, Solution.PartOne(input));
    }

    [TestMethod]
    public void TestPartTwo()
    {
        var input = """
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        """;

        Assert.AreEqual(281, Solution.PartTwo(input));
    }

}
