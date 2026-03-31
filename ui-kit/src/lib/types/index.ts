// 类型导出 - 为 ui-kit 包提供 TypeScript 类型支持

export interface ThemeTokens {
  colors: {
    primary: string;
    onPrimary: string;
    primaryContainer: string;
    onPrimaryContainer: string;
    secondary: string;
    onSecondary: string;
    secondaryContainer: string;
    onSecondaryContainer: string;
    tertiary: string;
    onTertiary: string;
    tertiaryContainer: string;
    onTertiaryContainer: string;
    error: string;
    onError: string;
    errorContainer: string;
    onErrorContainer: string;
    surface: string;
    onSurface: string;
    surfaceVariant: string;
    onSurfaceVariant: string;
    outline: string;
    outlineVariant: string;
    background: string;
    onBackground: string;
    inverseSurface: string;
    inverseOnSurface: string;
    inversePrimary: string;
  };
  typography: {
    fontFamily: string;
    fontSize: Record<string, string>;
    fontWeight: Record<string, number>;
    lineHeight: Record<string, number>;
  };
  spacing: Record<string, string>;
  radius: Record<string, string>;
  shadows: Record<string, string>;
}

export type ThemeMode = 'light' | 'dark' | 'system';

export interface ThemeOptions {
  mode?: ThemeMode;
  tokens?: Partial<ThemeTokens>;
}
