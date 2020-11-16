using System;
using System.Collections;
using System.Threading;

namespace mariner
{
    class Program
    {
        static void Main(string[] args)
        {
            foreach(DictionaryEntry ev in Environment.GetEnvironmentVariables()) {
                Console.WriteLine($"{ev.Key} = {ev.Value}");
            }

            while(true) {
                Thread.Sleep(1000);
                Console.Write(".");
            }
            Console.WriteLine("Hello World!");
        }
    }
}
