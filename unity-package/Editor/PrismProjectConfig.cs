using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace Prism.Editor
{
    internal static class PrismProjectConfig
    {
        internal const string ProjectFileName = ".prsmproject";
        internal const string LegacyProjectFileName = ".mnproject";
        internal const string DefaultCompilerPath = "prism";
        internal const string LegacyCompilerPath = "moonc";
        internal const string DefaultOutputDir = "Packages/com.prsm.generated/Runtime";
        internal const string LegacyOutputDir = "Packages/com.moon.generated/Runtime";

        internal static string FindProjectFilePath(string projectRoot)
        {
            string projectFile = Path.Combine(projectRoot, ProjectFileName);
            if (File.Exists(projectFile))
            {
                return projectFile;
            }

            string legacyProjectFile = Path.Combine(projectRoot, LegacyProjectFileName);
            return File.Exists(legacyProjectFile) ? legacyProjectFile : projectFile;
        }

        internal static string ResolveProjectPath(string projectRoot, string candidatePath)
        {
            if (string.IsNullOrWhiteSpace(candidatePath) || candidatePath == DefaultCompilerPath || candidatePath == LegacyCompilerPath)
            {
                return DefaultCompilerPath;
            }

            return Path.IsPathRooted(candidatePath)
                ? candidatePath
                : Path.GetFullPath(Path.Combine(projectRoot, candidatePath));
        }

        internal static bool IsPrismSourceAssetPath(string assetPath)
        {
            if (string.IsNullOrWhiteSpace(assetPath))
            {
                return false;
            }

            return assetPath.EndsWith(".prsm", StringComparison.OrdinalIgnoreCase)
                || assetPath.EndsWith(".mn", StringComparison.OrdinalIgnoreCase);
        }

        internal static string ParseTomlValue(string content, string key, string section = null)
        {
            return ParseTomlValue(content, new[] { key }, section);
        }

        internal static string ParseTomlValue(string content, IEnumerable<string> keys, string section = null)
        {
            if (string.IsNullOrWhiteSpace(content))
            {
                return null;
            }

            var keySet = new HashSet<string>(keys ?? Enumerable.Empty<string>(), StringComparer.Ordinal);
            if (keySet.Count == 0)
            {
                return null;
            }

            string[] lines = content.Split(new[] { "\r\n", "\n" }, StringSplitOptions.None);
            bool inSection = section == null;

            foreach (string rawLine in lines)
            {
                string trimmed = rawLine.Trim();

                if (trimmed.StartsWith("[") && trimmed.EndsWith("]"))
                {
                    string sectionName = trimmed.Substring(1, trimmed.Length - 2).Trim();
                    inSection = section == null || sectionName == section;
                    continue;
                }

                if (!inSection || string.IsNullOrEmpty(trimmed) || trimmed.StartsWith("#"))
                {
                    continue;
                }

                int eq = trimmed.IndexOf('=');
                if (eq <= 0)
                {
                    continue;
                }

                string parsedKey = trimmed.Substring(0, eq).Trim();
                if (!keySet.Contains(parsedKey))
                {
                    continue;
                }

                string value = trimmed.Substring(eq + 1).Trim();
                // Issue #79: strip inline `# comment` tail before the
                // quote handling. TOML's lexical rules treat `#` as the
                // start of a comment except inside a string literal, so
                // we walk the value tracking quote state and cut off
                // the first unquoted `#`. The old code simply trimmed
                // and then pattern-matched quote pairs, so a trailing
                // `# default` defeated the quote check and returned
                // the comment as part of the value.
                value = StripTomlInlineComment(value).Trim();
                if (value.StartsWith("\"") && value.EndsWith("\"") && value.Length >= 2)
                {
                    value = value.Substring(1, value.Length - 2);
                }
                return value;
            }

            return null;
        }

        /// Issue #79: strip a trailing `# comment` from a TOML value while
        /// respecting string-literal quoting. The `#` character is only a
        /// comment when it sits outside a `"..."` run; characters inside a
        /// string literal must survive unchanged.
        internal static string StripTomlInlineComment(string value)
        {
            if (string.IsNullOrEmpty(value))
            {
                return value;
            }
            bool inString = false;
            for (int i = 0; i < value.Length; i++)
            {
                char c = value[i];
                if (c == '"' && (i == 0 || value[i - 1] != '\\'))
                {
                    inString = !inString;
                    continue;
                }
                if (!inString && c == '#')
                {
                    return value.Substring(0, i).TrimEnd();
                }
            }
            return value;
        }

        internal static string NormalizeCompilerPath(string compilerPath)
        {
            if (string.IsNullOrWhiteSpace(compilerPath) || compilerPath == LegacyCompilerPath)
            {
                return DefaultCompilerPath;
            }

            return compilerPath;
        }

        internal static string NormalizeOutputDir(string outputDir)
        {
            if (string.IsNullOrWhiteSpace(outputDir) || string.Equals(outputDir, LegacyOutputDir, StringComparison.Ordinal))
            {
                return DefaultOutputDir;
            }

            return outputDir;
        }

        internal static string NormalizeProjectConfigContent(string content)
        {
            if (string.IsNullOrWhiteSpace(content))
            {
                return string.Empty;
            }

            string newline = content.Contains("\r\n", StringComparison.Ordinal) ? "\r\n" : "\n";
            string[] lines = content.Split(new[] { "\r\n", "\n" }, StringSplitOptions.None);
            var normalized = new List<string>(lines.Length);
            string currentSection = string.Empty;

            foreach (string rawLine in lines)
            {
                string trimmed = rawLine.Trim();
                if (trimmed.StartsWith("[") && trimmed.EndsWith("]"))
                {
                    currentSection = trimmed.Substring(1, trimmed.Length - 2).Trim();
                    normalized.Add(rawLine);
                    continue;
                }

                string line = rawLine;
                if (currentSection == "project")
                {
                    line = RenameKey(line, "moon_version", "prsm_version");
                }
                else if (currentSection == "compiler")
                {
                    line = NormalizeCompilerPathLine(line);
                    line = NormalizeOutputDirLine(line);
                }
                else if (currentSection == "source")
                {
                    line = NormalizeSourceIncludeLine(line);
                }

                normalized.Add(line);
            }

            return string.Join(newline, normalized);
        }

        private static string RenameKey(string rawLine, string oldKey, string newKey)
        {
            string trimmed = rawLine.TrimStart();
            if (!trimmed.StartsWith(oldKey, StringComparison.Ordinal))
            {
                return rawLine;
            }

            int separator = trimmed.IndexOf('=');
            if (separator < 0)
            {
                return rawLine;
            }

            string indent = rawLine.Substring(0, rawLine.Length - trimmed.Length);
            return indent + newKey + trimmed.Substring(separator);
        }

        private static string NormalizeCompilerPathLine(string rawLine)
        {
            string trimmed = rawLine.TrimStart();
            if (!trimmed.StartsWith("moonc_path", StringComparison.Ordinal)
                && !trimmed.StartsWith("prism_path", StringComparison.Ordinal))
            {
                return rawLine;
            }

            int separator = trimmed.IndexOf('=');
            if (separator < 0)
            {
                return rawLine;
            }

            string indent = rawLine.Substring(0, rawLine.Length - trimmed.Length);
            string value = trimmed.Substring(separator + 1).Trim().Trim('"');
            string normalizedValue = NormalizeCompilerPath(value);
            return indent + "prism_path = \"" + normalizedValue + "\"";
        }

        private static string NormalizeOutputDirLine(string rawLine)
        {
            string trimmed = rawLine.TrimStart();
            if (!trimmed.StartsWith("output_dir", StringComparison.Ordinal))
            {
                return rawLine;
            }

            int separator = trimmed.IndexOf('=');
            if (separator < 0)
            {
                return rawLine;
            }

            string indent = rawLine.Substring(0, rawLine.Length - trimmed.Length);
            string value = trimmed.Substring(separator + 1).Trim().Trim('"');
            string normalizedValue = NormalizeOutputDir(value);
            return indent + "output_dir = \"" + normalizedValue + "\"";
        }

        private static string NormalizeSourceIncludeLine(string rawLine)
        {
            string trimmed = rawLine.TrimStart();
            if (!trimmed.StartsWith("include", StringComparison.Ordinal))
            {
                return rawLine;
            }

            int separator = trimmed.IndexOf('=');
            if (separator < 0)
            {
                return rawLine;
            }

            string value = trimmed.Substring(separator + 1).Trim();
            if (!value.StartsWith("[", StringComparison.Ordinal) || !value.EndsWith("]", StringComparison.Ordinal))
            {
                return rawLine;
            }

            string body = value.Substring(1, value.Length - 2);
            var patterns = body
                .Split(new[] { ',' }, StringSplitOptions.RemoveEmptyEntries)
                .Select(item => item.Trim().Trim('"'))
                .Where(item => !string.IsNullOrWhiteSpace(item))
                .ToList();
            if (patterns.Count == 0)
            {
                return rawLine;
            }

            bool changed = false;
            for (int index = 0; index < patterns.Count; index++)
            {
                string pattern = patterns[index];
                if (!pattern.EndsWith(".mn", StringComparison.OrdinalIgnoreCase))
                {
                    continue;
                }

                string prsmPattern = pattern.Substring(0, pattern.Length - 3) + ".prsm";
                if (patterns.Contains(prsmPattern))
                {
                    continue;
                }

                patterns.Insert(index, prsmPattern);
                index++;
                changed = true;
            }

            if (!changed)
            {
                return rawLine;
            }

            string indent = rawLine.Substring(0, rawLine.Length - trimmed.Length);
            return indent + "include = [" + string.Join(", ", patterns.Select(pattern => "\"" + pattern + "\"")) + "]";
        }

        /// <summary>
        /// Build a complete .prsmproject TOML string from structured settings.
        /// </summary>
        internal static string BuildTomlContent(
            string name, string prsmVersion,
            string langVersion, string[] features,
            string compilerPath, string outputDir,
            string[] includes, string[] excludes,
            bool autoCompile, bool generateMeta, bool pascalCase,
            bool solidWarnings, int maxMethods, int maxDeps, int maxLength)
        {
            var sb = new System.Text.StringBuilder();

            sb.AppendLine("[project]");
            sb.AppendLine($"name = \"{name}\"");
            sb.AppendLine($"prsm_version = \"{prsmVersion}\"");
            sb.AppendLine();

            sb.AppendLine("[language]");
            sb.AppendLine($"version = \"{langVersion}\"");
            if (features != null && features.Length > 0)
            {
                sb.AppendLine("features = [" + string.Join(", ", features.Select(f => $"\"{f}\"")) + "]");
            }
            else
            {
                sb.AppendLine("features = []");
            }
            sb.AppendLine();

            sb.AppendLine("[compiler]");
            sb.AppendLine($"prism_path = \"{compilerPath}\"");
            sb.AppendLine($"output_dir = \"{outputDir}\"");
            sb.AppendLine();

            sb.AppendLine("[source]");
            sb.AppendLine("include = [" + string.Join(", ", (includes ?? new[] { "Assets/**/*.prsm" }).Select(s => $"\"{s}\"")) + "]");
            sb.AppendLine("exclude = [" + string.Join(", ", (excludes ?? Array.Empty<string>()).Select(s => $"\"{s}\"")) + "]");
            sb.AppendLine();

            sb.AppendLine("[features]");
            sb.AppendLine($"auto_compile_on_save = {(autoCompile ? "true" : "false")}");
            sb.AppendLine($"generate_meta_files = {(generateMeta ? "true" : "false")}");
            sb.AppendLine($"pascal_case_methods = {(pascalCase ? "true" : "false")}");
            sb.AppendLine();

            sb.AppendLine("[analysis]");
            sb.AppendLine($"solid_warnings = {(solidWarnings ? "true" : "false")}");
            sb.AppendLine($"max_public_methods = {maxMethods}");
            sb.AppendLine($"max_dependencies = {maxDeps}");
            sb.AppendLine($"max_method_length = {maxLength}");

            return sb.ToString();
        }

        /// <summary>
        /// Parse a TOML boolean value. Returns defaultValue if key not found.
        /// </summary>
        internal static bool ParseTomlBool(string content, string key, string section, bool defaultValue)
        {
            string value = ParseTomlValue(content, key, section);
            if (value == null) return defaultValue;
            return value.Trim().Equals("true", StringComparison.OrdinalIgnoreCase);
        }

        /// <summary>
        /// Parse a TOML integer value. Returns defaultValue if key not found.
        /// </summary>
        internal static int ParseTomlInt(string content, string key, string section, int defaultValue)
        {
            string value = ParseTomlValue(content, key, section);
            if (value == null) return defaultValue;
            return int.TryParse(value.Trim(), out int result) ? result : defaultValue;
        }

        /// <summary>
        /// Parse a TOML array of strings. Returns empty array if key not found.
        /// </summary>
        internal static string[] ParseTomlStringArray(string content, string key, string section)
        {
            string raw = ParseTomlValue(content, key, section);
            if (string.IsNullOrWhiteSpace(raw)) return Array.Empty<string>();
            // Strip outer brackets if present
            raw = raw.Trim();
            if (raw.StartsWith("[")) raw = raw.Substring(1);
            if (raw.EndsWith("]")) raw = raw.Substring(0, raw.Length - 1);
            return raw.Split(',')
                .Select(s => s.Trim().Trim('"'))
                .Where(s => !string.IsNullOrWhiteSpace(s))
                .ToArray();
        }
    }
}