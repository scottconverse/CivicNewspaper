using System;
using System.Diagnostics;
using System.IO;

public static class CivicSignShim
{
    private static string Quote(string value)
    {
        return "\"" + value.Replace("\"", "\\\"") + "\"";
    }

    public static int Main(string[] args)
    {
        if (args.Length != 1 || string.IsNullOrWhiteSpace(args[0]))
        {
            Console.Error.WriteLine("Expected exactly one artifact path.");
            return 2;
        }

        string root = Environment.GetEnvironmentVariable("GITHUB_WORKSPACE");
        if (string.IsNullOrWhiteSpace(root))
        {
            root = Environment.CurrentDirectory;
        }

        string script = Path.Combine(root, "scripts", "sign-windows-artifact.ps1");
        if (!File.Exists(script))
        {
            Console.Error.WriteLine("Signing script was not found in the workspace.");
            return 3;
        }

        var start = new ProcessStartInfo
        {
            FileName = "powershell.exe",
            Arguments = "-NoLogo -NoProfile -NonInteractive -ExecutionPolicy Bypass -File " +
                Quote(script) + " -File " + Quote(args[0]),
            UseShellExecute = false,
            CreateNoWindow = true,
            WorkingDirectory = root
        };

        using (Process process = Process.Start(start))
        {
            process.WaitForExit();
            return process.ExitCode;
        }
    }
}
