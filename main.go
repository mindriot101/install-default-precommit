package main

import (
	"flag"
	"fmt"
	"os"
	"path"
	"strings"

	"gopkg.in/yaml.v2"
)

type stage string

// stage enum
const (
	Commit           stage = "commit"
	MergeCommit            = "merge-commit"
	Push                   = "push"
	PrepareCommitMsg       = "prepare-commit-msg"
	CommitMsg              = "commit-msg"
	PostCheckout           = "post-checkout"
	PostCommit             = "post-commit"
	PostMerge              = "post-merge"
	PostRewrite            = "post-rewrite"
	Manual                 = "manual"
)

type hook struct {
	ID            string   `yaml:"id"`
	Name          string   `yaml:"name"`
	Entry         string   `yaml:"entry"`
	Language      string   `yaml:"language"`
	AlwaysRun     bool     `yaml:"always_run"`
	Verbose       bool     `yaml:"verbose"`
	PassFilenames bool     `yaml:"pass_filenames"`
	Stages        []stage  `yaml:"stages"`
	Types         []string `yaml:"types"`
	Files         string   `yaml:"files"`
}

type repo struct {
	Name  string `yaml:"repo"`
	Hooks []hook `yaml:"hooks"`
}

type config struct {
	Repos []repo `yaml:"repos"`
}

func pythonConfig() config {
	return config{
		Repos: []repo{
			{
				Name: "local",
				Hooks: []hook{
					{
						ID:       "cfn-lint",
						Name:     "Lint cloudformation",
						Entry:    "cfn-lint",
						Language: "system",
						Files:    "cloudformation.yml",
						Stages:   []stage{Commit},
					},
					{
						ID:            "gitlab-yaml",
						Name:          "Check gitlab yaml",
						Entry:         "lab ci lint",
						Language:      "system",
						Files:         ".gitlab-ci.yml",
						PassFilenames: false,
						Stages:        []stage{Commit},
					},
					{
						ID:            "flake8",
						Name:          "flake8",
						Entry:         "flake8",
						Language:      "system",
						PassFilenames: false,
						Types:         []string{"python"},
						Stages:        []stage{Commit},
					},
					{
						ID:            "pytest",
						Name:          "pytest",
						Entry:         "pytest -n auto --quiet",
						Language:      "system",
						Types:         []string{"python"},
						PassFilenames: false,
						Stages:        []stage{Commit, Push},
					},
				},
			},
		},
	}
}

func rustConfig() config {
	return config{
		Repos: []repo{
			{
				Name: "local",
				Hooks: []hook{
					{
						ID:            "cargo test",
						Name:          "cargo test",
						Entry:         "cargo test",
						Language:      "system",
						AlwaysRun:     true,
						PassFilenames: false,
						Stages:        []stage{Commit, Push},
					},
				},
			},
		},
	}
}

func goConfig() config {
	return config{
		Repos: []repo{
			{
				Name: "local",
				Hooks: []hook{
					{
						ID:            "go test",
						Name:          "go test",
						Entry:         "go test",
						Language:      "system",
						AlwaysRun:     true,
						PassFilenames: false,
						Stages:        []stage{Commit, Push},
					},
				},
			},
		},
	}
}

func findProjectRoot() (string, error) {
	dir, err := os.Getwd()
	if err != nil {
		return "", fmt.Errorf("cannot get current working directory: %w", err)
	}
	for {
		if dir == "/" {
			return "", fmt.Errorf("could not find git directory up to filesystem root")
		}

		if pathExists(path.Join(dir, ".git")) {
			return dir, nil
		}
		dir = path.Join(dir, "..")
	}
}

func pathExists(path string) bool {
	_, err := os.Stat(path)
	return err == nil
}

func main() {
	var langFlag = flag.String("lang", "", "Language to install hooks for")
	var forceFlag = flag.Bool("f", false, "Force overwriting existing files")
	flag.Parse()

	if *langFlag == "" {
		fmt.Fprintln(os.Stderr, "Language argument is required")
		os.Exit(1)
	}

	projectRoot, err := findProjectRoot()
	if err != nil {
		panic(err)
	}
	outFname := path.Join(projectRoot, ".pre-commit-config.yaml")
	if pathExists(outFname) && !*forceFlag {
		fmt.Fprintf(os.Stderr, "file %s exists, exiting\n", outFname)
		os.Exit(0)
	}

	language := strings.ToLower(*langFlag)

	var cfg config
	switch language {
	case "python":
		cfg = pythonConfig()
	case "rust":
		cfg = rustConfig()
	case "go":
		cfg = goConfig()
	default:
		panic("Not implemented")
	}

	b, err := yaml.Marshal(&cfg)
	if err != nil {
		panic(err)
	}
	f, err := os.Create(outFname)
	if err != nil {
		panic(err)
	}
	if _, err = f.Write(b); err != nil {
		panic(err)
	}
}
