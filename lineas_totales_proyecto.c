#include <stdio.h>

#ifdef _WIN32
#include <windows.h>
#define sleep(x) Sleep((x) * 1000)
#else
#include <unistd.h>
#endif

// Variable externa que será actualizada desde Rust
extern int total_code_lines;

int main(void)
{
    printf("╔═══════════════════════════════════════════════════════════╗\n");
    printf("║          ESTADÍSTICAS DEL PROYECTO CODE TRANSLATOR       ║\n");
    printf("╚═══════════════════════════════════════════════════════════╝\n\n");
    
    printf("📊 Total de líneas de código en el proyecto: %d líneas\n\n", total_code_lines);
    
    printf("📁 Archivos analizados:\n");
    printf("   • Archivos Rust (.rs)\n");
    printf("   • Archivos C (.c, .h)\n");
    printf("   • Archivos de configuración (.toml, .yml, .gradle, .xml)\n");
    printf("   • Archivos de documentación (.md)\n\n");
    
    printf("⏳ Esta ventana se cerrará en 5 segundos...\n");
    sleep(5);
    
    return 0;
}
