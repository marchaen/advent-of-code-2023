namespace Day07.Solutions;

internal enum Card
{
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    T,
    J,
    Q,
    K,
    A
}

internal enum HandType
{
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

internal class CardMethods
{
    internal static Card? TryParse(char input) =>
        input switch
        {
            '2' => Card.TWO,
            '3' => Card.THREE,
            '4' => Card.FOUR,
            '5' => Card.FIVE,
            '6' => Card.SIX,
            '7' => Card.SEVEN,
            '8' => Card.EIGHT,
            '9' => Card.NINE,
            'T' => Card.T,
            'J' => Card.J,
            'Q' => Card.Q,
            'K' => Card.K,
            'A' => Card.A,
            _ => null
        };

    internal static HandType DetermineHandType(Card[] cards)
    {
        var cardCounts = cards
            .GroupBy((card) => card)
            .ToDictionary((group) => group.Key, (group) => group.Count());

        return cardCounts.Count() switch
        {
            1 => HandType.FiveOfAKind,
            2 => cardCounts.Values.Contains(4) ? HandType.FourOfAKind : HandType.FullHouse,
            3 => cardCounts.Values.Contains(3) ? HandType.ThreeOfAKind : HandType.TwoPair,
            4 => HandType.OnePair,
            5 => HandType.HighCard,
            _ => throw new Exception("Determining the hand type only works with exactly five cards")
        };
    }
}

internal class Hand : IComparable<Hand>
{
    Card[] cards;
    int bid;
    HandType handType;

    public int WeigthedBid(int listIndex) => this.bid * (listIndex + 1);

    internal static Hand TryParseFrom(string input)
    {
        // Example input:
        // 32T3K 765

        var inputTrimmed = input.Trim();
        var rawCards = inputTrimmed.Substring(0, 5);
        var rawBid = inputTrimmed.Substring(6);

        var cards = rawCards
            .ToCharArray()
            .Select(
                (rawCard) =>
                {
                    var maybeCard = CardMethods.TryParse(rawCard);

                    if (maybeCard is Card card)
                        return card;

                    throw new ArgumentException($"'{rawCard}' is not a valid card");
                }
            )
            .ToArray();

        return new Hand(cards, int.Parse(rawBid));
    }

    internal Hand(Card[] cards, int bid)
    {
        if (cards.Count() != 5)
            throw new ArgumentException("Each hand needs to have exactly five cards");

        this.cards = cards;
        this.bid = bid;
        this.handType = CardMethods.DetermineHandType(cards);
    }

    public int CompareTo(Hand? other)
    {
        if (other is null)
            return 1;

        var typeComparison = this.handType.CompareTo(other.handType);

        if (typeComparison != 0)
            return typeComparison;

        for (int card = 0; card < 5; card++)
        {
            var cardComparison = this.cards[card].CompareTo(other.cards[card]);

            if (cardComparison != 0)
                return cardComparison;
        }

        return 0;
    }
}
