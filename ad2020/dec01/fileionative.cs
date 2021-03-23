using System.Collections.Generic;
using System.IO.File;

namespace @__default {

public partial class FileIO
{
    public int[] ReadFileOfNumbersNative(char[] filename)
    {
        List<int> numbers = new List<int>();
        foreach (string line in File.ReadLines(new string(filename)))
        {
            numbers.Add(int.Parse(line));
        }
        return numbers.ToArray();
    }
}

}
