@echo off
setlocal

:: ============================================================================
:: Kuormitustestin suorittaja
::
:: Tämä skripti käynnistää määritellyn määrän teststamp.exe-ohjelmia
:: rinnakkain kuormitustestiä varten.
::
:: Käyttö:
::   run_load_test.bat [ohjelmien_määrä] [leimausten_määrä_per_ohjelma]
::
:: Esimerkki (käynnistää 5 ohjelmaa, joista kukin tekee 100 leimausta):
::   run_load_test.bat 5 100
:: ============================================================================

set INSTANCE_COUNT=%1
set STAMP_COUNT=%2

if "%INSTANCE_COUNT%"=="" ( goto :usage )
if "%STAMP_COUNT%"=="" ( goto :usage )

echo Aloitetaan %INSTANCE_COUNT% kpl teststamp.exe ohjelmaa.
echo Jokainen ohjelma suorittaa %STAMP_COUNT% leimausta.
echo.

for /L %%i in (1, 1, %INSTANCE_COUNT%) do (
    echo Kaynnistetaan ohjelma nro %%i...
    start "TestStamp %%i" "%~dp0target\debug\teststamp.exe" %STAMP_COUNT% %%i
)

goto :eof

:usage
echo Virhe: Anna vaaditut parametrit.
echo.
echo Kaytto: %~n0 [ohjelmien_maara] [leimausten_maara_per_ohjelma]
echo Esimerkki: %~n0 5 100

endlocal