namespace Day07.Tests;

using Day07.Solutions;

[TestClass]
public class Tests
{
    [TestMethod]
    public void PartOne()
    {
        var input = """
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        """;

        Assert.AreEqual(6440, Solution.PartOne(input));
    }
}

