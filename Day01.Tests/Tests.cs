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
}
