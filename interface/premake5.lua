project "equality"
    kind "StaticLib"
    language "C"
    systemversion "latest"
    staticruntime "on"

    targetdir ("binaries/" .. outputdir .. "/%{prj.name}")
    objdir ("intermediates/" .. outputdir .. "/%{prj.name}")

    files {
        "/source/**.h",
        "/source/**.c"
    }

    includedirs {
        "/source/include"
    }

    filter { "system:windows", "configurations:Release" }
        buildoptions "/MT"
