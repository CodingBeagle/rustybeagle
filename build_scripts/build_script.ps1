Param(
    [string]$workingDirectory    
)

Copy-Item -Path "${workingDirectory}\shaders" -Recurse -Destination "${workingDirectory}\target\debug\resources" -Force