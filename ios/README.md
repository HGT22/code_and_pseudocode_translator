# iOS / App Store (Code Translator)

Este repositorio genera en release el archivo `code-translator-ios.xcframework.zip`.

## Qué puedes hacer desde Windows

- Crear el framework iOS automáticamente con GitHub Actions.
- Descargar el `XCFramework` listo para integrar.

## Qué necesitas en Mac (obligatorio para App Store)

Apple exige Xcode + certificados para generar y subir `.ipa`.

1. En un Mac, abre Xcode y crea una app iOS (SwiftUI o UIKit).
2. Añade `code-translator-ios.xcframework` al proyecto.
3. Usa las funciones C exportadas:
   - `ct_translate`
   - `ct_free_string`
4. Configura `Signing & Capabilities` con tu Apple Developer Team.
5. Haz `Archive` y sube con Organizer a App Store Connect.

## API C expuesta por Rust

Header: `ios/include/code_translator.h`

```c
char *ct_translate(const char *source_code, const char *source_language, const char *target_language);
void ct_free_string(char *ptr);
```

## Flujo recomendado de release

1. Publica tag (por ejemplo `v0.1.2`).
2. GitHub Actions adjunta `code-translator-ios.xcframework.zip` al release.
3. Descárgalo en Mac y úsalo en la app iOS.
4. Sube la app a TestFlight y luego a App Store.

## Nota importante

No existe forma soportada por Apple de publicar directamente en App Store solo desde Windows. El paso final siempre requiere entorno Apple.
