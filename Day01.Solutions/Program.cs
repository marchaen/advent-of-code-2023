﻿using Day01.Solutions;

var input = File.ReadAllText(AppDomain.CurrentDomain.BaseDirectory + "./Day01.txt");

Console.WriteLine("Solution for part one: " + Solution.PartOne(input));
Console.WriteLine("Solution for part two: " + Solution.PartTwo(input));
