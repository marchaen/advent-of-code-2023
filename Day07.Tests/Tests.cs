namespace Day07.Tests;

using Day07.Solutions;

[TestClass]
public class Tests
{
    const string INPUT = """
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    """;

    [TestMethod]
    public void PartOne()
    {
        Assert.AreEqual(6440, Solution.PartOne(INPUT));
    }

    [TestMethod]
    public void PartTwo()
    {
        Assert.AreEqual(5905, Solution.PartTwo(INPUT));
    }
}
