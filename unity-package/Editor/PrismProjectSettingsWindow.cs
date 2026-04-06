using System.IO;
using System.Linq;
using UnityEditor;
using UnityEngine;

namespace Prism.Editor
{
    /// <summary>
    /// EditorWindow for editing .prsmproject settings via GUI.
    /// Opened via Window > PrSM > Project Settings.
    /// </summary>
    public class PrismProjectSettingsWindow : EditorWindow
    {
        private static readonly string[] LanguageVersions = { "1", "2", "3" };
        private static readonly string[] FeatureNames =
        {
            "pattern-bindings", "input-system", "auto-unlisten",
            "interface", "generics", "singleton", "pool", "solid-analysis", "optimizer"
        };

        // Settings state
        private string projectName = "";
        private string prsmVersion = "0.1.0";
        private int languageVersionIndex = 2; // default "3"
        private bool[] featureFlags = new bool[9];
        private string compilerPath = "prism";
        private string outputDir = PrismProjectConfig.DefaultOutputDir;
        private string includePatterns = "Assets/**/*.prsm";
        private string excludePatterns = "";
        private bool autoCompileOnSave = true;
        private bool generateMetaFiles = true;
        private bool pascalCaseMethods = true;
        private bool solidWarnings = true;
        private int maxPublicMethods = 8;
        private int maxDependencies = 6;
        private int maxMethodLength = 50;

        private Vector2 scrollPosition;
        private bool isDirty;
        private bool loaded;

        [MenuItem("Window/PrSM/Project Settings")]
        public static void ShowWindow()
        {
            var window = GetWindow<PrismProjectSettingsWindow>("PrSM Settings");
            window.minSize = new Vector2(420, 550);
        }

        private void OnEnable()
        {
            LoadSettings();
        }

        private void LoadSettings()
        {
            string filePath = PrismProjectSettings.GetActiveProjectFilePath();
            if (!File.Exists(filePath))
            {
                loaded = true;
                return;
            }

            string content = File.ReadAllText(filePath);

            projectName = PrismProjectConfig.ParseTomlValue(content, "name", "project") ?? "";
            prsmVersion = PrismProjectConfig.ParseTomlValue(content, "prsm_version", "project") ?? "0.1.0";

            string langVer = PrismProjectConfig.ParseTomlValue(content, "version", "language") ?? "3";
            languageVersionIndex = System.Array.IndexOf(LanguageVersions, langVer);
            if (languageVersionIndex < 0) languageVersionIndex = 2;

            string[] features = PrismProjectConfig.ParseTomlStringArray(content, "features", "language");
            for (int i = 0; i < FeatureNames.Length; i++)
            {
                featureFlags[i] = System.Array.Exists(features, f => f == FeatureNames[i]);
            }

            compilerPath = PrismProjectConfig.ParseTomlValue(content, new[] { "prism_path", "moonc_path" }, "compiler") ?? "prism";
            outputDir = PrismProjectConfig.ParseTomlValue(content, "output_dir", "compiler") ?? PrismProjectConfig.DefaultOutputDir;

            string[] includes = PrismProjectConfig.ParseTomlStringArray(content, "include", "source");
            includePatterns = includes.Length > 0 ? string.Join(", ", includes) : "Assets/**/*.prsm";
            string[] excludes = PrismProjectConfig.ParseTomlStringArray(content, "exclude", "source");
            excludePatterns = excludes.Length > 0 ? string.Join(", ", excludes) : "";

            autoCompileOnSave = PrismProjectConfig.ParseTomlBool(content, "auto_compile_on_save", "features", true);
            generateMetaFiles = PrismProjectConfig.ParseTomlBool(content, "generate_meta_files", "features", true);
            pascalCaseMethods = PrismProjectConfig.ParseTomlBool(content, "pascal_case_methods", "features", true);

            solidWarnings = PrismProjectConfig.ParseTomlBool(content, "solid_warnings", "analysis", true);
            maxPublicMethods = PrismProjectConfig.ParseTomlInt(content, "max_public_methods", "analysis", 8);
            maxDependencies = PrismProjectConfig.ParseTomlInt(content, "max_dependencies", "analysis", 6);
            maxMethodLength = PrismProjectConfig.ParseTomlInt(content, "max_method_length", "analysis", 50);

            isDirty = false;
            loaded = true;
        }

        private void OnGUI()
        {
            if (!loaded) LoadSettings();

            scrollPosition = EditorGUILayout.BeginScrollView(scrollPosition);

            EditorGUI.BeginChangeCheck();

            DrawSection("Project", DrawProjectSection);
            DrawSection("Language", DrawLanguageSection);
            DrawSection("Compiler", DrawCompilerSection);
            DrawSection("Source", DrawSourceSection);
            DrawSection("Build Features", DrawBuildFeaturesSection);
            DrawSection("Analysis", DrawAnalysisSection);

            if (EditorGUI.EndChangeCheck())
            {
                isDirty = true;
            }

            EditorGUILayout.EndScrollView();

            EditorGUILayout.Space(8);
            DrawButtons();
        }

        private void DrawSection(string title, System.Action drawContent)
        {
            EditorGUILayout.Space(6);
            EditorGUILayout.LabelField(title, EditorStyles.boldLabel);
            EditorGUI.indentLevel++;
            drawContent();
            EditorGUI.indentLevel--;
        }

        private void DrawProjectSection()
        {
            projectName = EditorGUILayout.TextField("Name", projectName);
            prsmVersion = EditorGUILayout.TextField("PrSM Version", prsmVersion);
        }

        private void DrawLanguageSection()
        {
            languageVersionIndex = EditorGUILayout.Popup("Version", languageVersionIndex, LanguageVersions);

            EditorGUILayout.LabelField("Features", EditorStyles.miniLabel);
            EditorGUI.indentLevel++;
            for (int i = 0; i < FeatureNames.Length; i++)
            {
                featureFlags[i] = EditorGUILayout.Toggle(FeatureNames[i], featureFlags[i]);
            }
            EditorGUI.indentLevel--;
        }

        private void DrawCompilerSection()
        {
            EditorGUILayout.BeginHorizontal();
            compilerPath = EditorGUILayout.TextField("Compiler Path", compilerPath);
            if (GUILayout.Button("Browse", GUILayout.Width(60)))
            {
                string path = EditorUtility.OpenFilePanel("Select PrSM Compiler", "", "exe");
                if (!string.IsNullOrEmpty(path))
                {
                    compilerPath = path;
                    isDirty = true;
                }
            }
            EditorGUILayout.EndHorizontal();

            EditorGUILayout.BeginHorizontal();
            outputDir = EditorGUILayout.TextField("Output Directory", outputDir);
            if (GUILayout.Button("Browse", GUILayout.Width(60)))
            {
                string path = EditorUtility.OpenFolderPanel("Select Output Directory", "", "");
                if (!string.IsNullOrEmpty(path))
                {
                    // Make relative to project root
                    string projectRoot = PrismProjectSettings.GetProjectRoot();
                    if (path.StartsWith(projectRoot))
                    {
                        path = path.Substring(projectRoot.Length).TrimStart(Path.DirectorySeparatorChar, Path.AltDirectorySeparatorChar);
                    }
                    outputDir = path;
                    isDirty = true;
                }
            }
            EditorGUILayout.EndHorizontal();
        }

        private void DrawSourceSection()
        {
            includePatterns = EditorGUILayout.TextField("Include Patterns", includePatterns);
            excludePatterns = EditorGUILayout.TextField("Exclude Patterns", excludePatterns);
            EditorGUILayout.HelpBox("Comma-separated glob patterns. Example: Assets/**/*.prsm", MessageType.None);
        }

        private void DrawBuildFeaturesSection()
        {
            autoCompileOnSave = EditorGUILayout.Toggle("Auto Compile on Save", autoCompileOnSave);
            generateMetaFiles = EditorGUILayout.Toggle("Generate Meta Files", generateMetaFiles);
            pascalCaseMethods = EditorGUILayout.Toggle("PascalCase Methods", pascalCaseMethods);
        }

        private void DrawAnalysisSection()
        {
            solidWarnings = EditorGUILayout.Toggle("SOLID Warnings", solidWarnings);

            EditorGUI.BeginDisabledGroup(!solidWarnings);
            maxPublicMethods = EditorGUILayout.IntField("Max Public Methods", maxPublicMethods);
            maxDependencies = EditorGUILayout.IntField("Max Dependencies", maxDependencies);
            maxMethodLength = EditorGUILayout.IntField("Max Method Length", maxMethodLength);
            EditorGUI.EndDisabledGroup();
        }

        private void DrawButtons()
        {
            EditorGUILayout.BeginHorizontal();

            EditorGUI.BeginDisabledGroup(!isDirty);
            if (GUILayout.Button("Save", GUILayout.Height(28)))
            {
                SaveSettings();
            }
            if (GUILayout.Button("Revert", GUILayout.Height(28)))
            {
                LoadSettings();
                isDirty = false;
            }
            EditorGUI.EndDisabledGroup();

            if (GUILayout.Button("Open .prsmproject", GUILayout.Height(28)))
            {
                string filePath = PrismProjectSettings.GetActiveProjectFilePath();
                if (File.Exists(filePath))
                {
                    EditorUtility.OpenWithDefaultApp(filePath);
                }
            }

            EditorGUILayout.EndHorizontal();

            if (isDirty)
            {
                EditorGUILayout.HelpBox("Unsaved changes.", MessageType.Warning);
            }
        }

        private void SaveSettings()
        {
            // Collect features
            var enabledFeatures = new System.Collections.Generic.List<string>();
            for (int i = 0; i < FeatureNames.Length; i++)
            {
                if (featureFlags[i]) enabledFeatures.Add(FeatureNames[i]);
            }

            // Parse patterns from comma-separated
            string[] includes = ParseCommaSeparated(includePatterns);
            string[] excludes = ParseCommaSeparated(excludePatterns);

            string toml = PrismProjectConfig.BuildTomlContent(
                projectName, prsmVersion,
                LanguageVersions[languageVersionIndex], enabledFeatures.ToArray(),
                compilerPath, outputDir,
                includes, excludes,
                autoCompileOnSave, generateMetaFiles, pascalCaseMethods,
                solidWarnings, maxPublicMethods, maxDependencies, maxMethodLength
            );

            string filePath = PrismProjectSettings.GetProjectFilePath();
            File.WriteAllText(filePath, toml);
            PrismProjectSettings.ClearCache();

            isDirty = false;
            Debug.Log($"[PrSM] Project settings saved to {filePath}");
        }

        private static string[] ParseCommaSeparated(string input)
        {
            if (string.IsNullOrWhiteSpace(input)) return System.Array.Empty<string>();
            return input.Split(',')
                .Select(s => s.Trim())
                .Where(s => !string.IsNullOrWhiteSpace(s))
                .ToArray();
        }
    }
}
