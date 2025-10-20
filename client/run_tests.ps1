# Script para executar os testes k6 3 vezes
# Salva os resultados com o padrão: out(${numero da repeticao})_${linguagem}.txt

$language = "csharp"  # Linguagem sendo testada

Write-Host "Iniciando execução dos testes k6..." -ForegroundColor Green
Write-Host "Linguagem: $language" -ForegroundColor Yellow

for ($i = 1; $i -le 3; $i++) {
    $outputFile = "out($i)_$language.txt"
    
    Write-Host "`nExecutando teste $i de 3..." -ForegroundColor Cyan
    Write-Host "Arquivo de saída: $outputFile" -ForegroundColor Gray
    
    # Executa o k6 e salva a saída no arquivo
    k6 run .\script.js > $outputFile
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "Teste $i concluído com sucesso!" -ForegroundColor Green
    } else {
        Write-Host "Erro na execução do teste $i" -ForegroundColor Red
    }
    
    # Pequena pausa entre os testes
    Start-Sleep -Seconds 5
    # Limpa o console
    Clear-Host
}

Write-Host "`nTodos os testes foram executados!" -ForegroundColor Green
Write-Host "Arquivos gerados:" -ForegroundColor Yellow
Get-ChildItem -Path "out(*)_$language.txt" | ForEach-Object { Write-Host "  - $($_.Name)" -ForegroundColor White }
