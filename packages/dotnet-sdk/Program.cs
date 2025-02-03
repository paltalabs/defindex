using System;
using System.Threading;

class Program
{
    static void Main(string[] args)
    {
        for (int i = 0; i < 5; i++)
        {
            Console.WriteLine($"Test output {i}");
            Thread.Sleep(1000); // Wait 1 second between outputs
        }
    }
} 