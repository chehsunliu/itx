// Package config provides helpers for loading application.yaml-style configuration
// with ${VAR} / ${VAR:default} substitution.
package config

import (
	"os"
	"regexp"
)

var envVarRe = regexp.MustCompile(`\$\{([A-Za-z_][A-Za-z0-9_]*)(:([^}]*))?\}`)

// SubstituteEnv replaces ${VAR} and ${VAR:default} placeholders in raw using
// process env vars. A missing var with no default substitutes to the empty string.
func SubstituteEnv(raw string) string {
	return envVarRe.ReplaceAllStringFunc(raw, func(match string) string {
		parts := envVarRe.FindStringSubmatch(match)
		name := parts[1]
		hasDefault := parts[2] != ""
		defaultVal := parts[3]
		if v, ok := os.LookupEnv(name); ok {
			return v
		}
		if hasDefault {
			return defaultVal
		}
		return ""
	})
}
