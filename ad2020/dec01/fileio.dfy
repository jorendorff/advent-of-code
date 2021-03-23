// Useful to convert Dafny strings into arrays of characters.
method ToArray<A>(s: seq<A>) returns (a: array<A>)
    ensures a[..] == s
{
    a := new A[|s|](i requires 0 <= i < |s| => s[i]);
}

class FileIO
{
    static method ReadFileOfNumbers(name: string)
        returns (numbers: array<nat>)
    {
        var nameChars := ToArray(name);
        numbers := FileIO.ReadFileOfNumbersNative(nameChars);
        //numbers := new nat[nums32.Length](i reads nums32 requires 0 <= i < nums32.Length => nums32[i] as nat);
    }

    static method{:axiom} ReadFileOfNumbersNative(name: array<char>)
        returns (numbers: array<nat>)
        ensures fresh(numbers)
}
