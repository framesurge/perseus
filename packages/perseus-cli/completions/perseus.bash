_perseus() {
    local i cur prev opts cmd
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${cmd},${i}" in
            ",$1")
                cmd="perseus"
                ;;
            perseus,build)
                cmd="perseus__build"
                ;;
            perseus,check)
                cmd="perseus__check"
                ;;
            perseus,clean)
                cmd="perseus__clean"
                ;;
            perseus,deploy)
                cmd="perseus__deploy"
                ;;
            perseus,export)
                cmd="perseus__export"
                ;;
            perseus,export-error-page)
                cmd="perseus__export__error__page"
                ;;
            perseus,help)
                cmd="perseus__help"
                ;;
            perseus,init)
                cmd="perseus__init"
                ;;
            perseus,new)
                cmd="perseus__new"
                ;;
            perseus,serve)
                cmd="perseus__serve"
                ;;
            perseus,snoop)
                cmd="perseus__snoop"
                ;;
            perseus,test)
                cmd="perseus__test"
                ;;
            perseus,tinker)
                cmd="perseus__tinker"
                ;;
            perseus__help,build)
                cmd="perseus__help__build"
                ;;
            perseus__help,check)
                cmd="perseus__help__check"
                ;;
            perseus__help,clean)
                cmd="perseus__help__clean"
                ;;
            perseus__help,deploy)
                cmd="perseus__help__deploy"
                ;;
            perseus__help,export)
                cmd="perseus__help__export"
                ;;
            perseus__help,export-error-page)
                cmd="perseus__help__export__error__page"
                ;;
            perseus__help,help)
                cmd="perseus__help__help"
                ;;
            perseus__help,init)
                cmd="perseus__help__init"
                ;;
            perseus__help,new)
                cmd="perseus__help__new"
                ;;
            perseus__help,serve)
                cmd="perseus__help__serve"
                ;;
            perseus__help,snoop)
                cmd="perseus__help__snoop"
                ;;
            perseus__help,test)
                cmd="perseus__help__test"
                ;;
            perseus__help,tinker)
                cmd="perseus__help__tinker"
                ;;
            perseus__help__snoop,build)
                cmd="perseus__help__snoop__build"
                ;;
            perseus__help__snoop,serve)
                cmd="perseus__help__snoop__serve"
                ;;
            perseus__help__snoop,wasm-build)
                cmd="perseus__help__snoop__wasm__build"
                ;;
            perseus__snoop,build)
                cmd="perseus__snoop__build"
                ;;
            perseus__snoop,help)
                cmd="perseus__snoop__help"
                ;;
            perseus__snoop,serve)
                cmd="perseus__snoop__serve"
                ;;
            perseus__snoop,wasm-build)
                cmd="perseus__snoop__wasm__build"
                ;;
            perseus__snoop__help,build)
                cmd="perseus__snoop__help__build"
                ;;
            perseus__snoop__help,help)
                cmd="perseus__snoop__help__help"
                ;;
            perseus__snoop__help,serve)
                cmd="perseus__snoop__help__serve"
                ;;
            perseus__snoop__help,wasm-build)
                cmd="perseus__snoop__help__wasm__build"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        perseus)
            opts="-h -V --cargo-engine-path --cargo-browser-path --wasm-bindgen-path --wasm-opt-path --rustup-path --wasm-release-rustflags --cargo-engine-args --cargo-browser-args --wasm-bindgen-args --wasm-opt-args --git-path --reload-server-host --reload-server-port --sequential --no-browser-reload --wasm-bindgen-version --wasm-opt-version --no-system-tools-cache --verbose --disable-bundle-compression --help --version build export-error-page export serve test clean deploy tinker snoop new init check help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --cargo-engine-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-browser-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --rustup-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-release-rustflags)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-engine-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-browser-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --git-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reload-server-host)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reload-server-port)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__build)
            opts="-w -h --release --watch --custom-watch --cargo-engine-path --cargo-browser-path --wasm-bindgen-path --wasm-opt-path --rustup-path --wasm-release-rustflags --cargo-engine-args --cargo-browser-args --wasm-bindgen-args --wasm-opt-args --git-path --reload-server-host --reload-server-port --sequential --no-browser-reload --wasm-bindgen-version --wasm-opt-version --no-system-tools-cache --verbose --disable-bundle-compression --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --custom-watch)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-engine-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-browser-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --rustup-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-release-rustflags)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-engine-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-browser-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --git-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reload-server-host)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reload-server-port)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__check)
            opts="-w -g -h --watch --custom-watch --generate --cargo-engine-path --cargo-browser-path --wasm-bindgen-path --wasm-opt-path --rustup-path --wasm-release-rustflags --cargo-engine-args --cargo-browser-args --wasm-bindgen-args --wasm-opt-args --git-path --reload-server-host --reload-server-port --sequential --no-browser-reload --wasm-bindgen-version --wasm-opt-version --no-system-tools-cache --verbose --disable-bundle-compression --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --custom-watch)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-engine-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-browser-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --rustup-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-release-rustflags)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-engine-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-browser-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --git-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reload-server-host)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reload-server-port)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__clean)
            opts="-h --cargo-engine-path --cargo-browser-path --wasm-bindgen-path --wasm-opt-path --rustup-path --wasm-release-rustflags --cargo-engine-args --cargo-browser-args --wasm-bindgen-args --wasm-opt-args --git-path --reload-server-host --reload-server-port --sequential --no-browser-reload --wasm-bindgen-version --wasm-opt-version --no-system-tools-cache --verbose --disable-bundle-compression --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --cargo-engine-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-browser-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --rustup-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-release-rustflags)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-engine-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-browser-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --git-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reload-server-host)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reload-server-port)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__deploy)
            opts="-o -e -h --output --export-static --no-minify-js --cargo-engine-path --cargo-browser-path --wasm-bindgen-path --wasm-opt-path --rustup-path --wasm-release-rustflags --cargo-engine-args --cargo-browser-args --wasm-bindgen-args --wasm-opt-args --git-path --reload-server-host --reload-server-port --sequential --no-browser-reload --wasm-bindgen-version --wasm-opt-version --no-system-tools-cache --verbose --disable-bundle-compression --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-engine-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-browser-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --rustup-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-release-rustflags)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-engine-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-browser-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --git-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reload-server-host)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reload-server-port)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__export)
            opts="-s -w -h --release --serve --host --port --watch --custom-watch --cargo-engine-path --cargo-browser-path --wasm-bindgen-path --wasm-opt-path --rustup-path --wasm-release-rustflags --cargo-engine-args --cargo-browser-args --wasm-bindgen-args --wasm-opt-args --git-path --reload-server-host --reload-server-port --sequential --no-browser-reload --wasm-bindgen-version --wasm-opt-version --no-system-tools-cache --verbose --disable-bundle-compression --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --host)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --port)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --custom-watch)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-engine-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-browser-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --rustup-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-release-rustflags)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-engine-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-browser-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --git-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reload-server-host)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reload-server-port)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__export__error__page)
            opts="-c -o -h --code --output --cargo-engine-path --cargo-browser-path --wasm-bindgen-path --wasm-opt-path --rustup-path --wasm-release-rustflags --cargo-engine-args --cargo-browser-args --wasm-bindgen-args --wasm-opt-args --git-path --reload-server-host --reload-server-port --sequential --no-browser-reload --wasm-bindgen-version --wasm-opt-version --no-system-tools-cache --verbose --disable-bundle-compression --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --code)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-engine-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-browser-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --rustup-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-release-rustflags)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-engine-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-browser-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --git-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reload-server-host)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reload-server-port)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__help)
            opts="build export-error-page export serve test clean deploy tinker snoop new init check help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__help__build)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__help__check)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__help__clean)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__help__deploy)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__help__export)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__help__export__error__page)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__help__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__help__init)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__help__new)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__help__serve)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__help__snoop)
            opts="build wasm-build serve"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__help__snoop__build)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__help__snoop__serve)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__help__snoop__wasm__build)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__help__test)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__help__tinker)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__init)
            opts="-h --cargo-engine-path --cargo-browser-path --wasm-bindgen-path --wasm-opt-path --rustup-path --wasm-release-rustflags --cargo-engine-args --cargo-browser-args --wasm-bindgen-args --wasm-opt-args --git-path --reload-server-host --reload-server-port --sequential --no-browser-reload --wasm-bindgen-version --wasm-opt-version --no-system-tools-cache --verbose --disable-bundle-compression --help <NAME>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --cargo-engine-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-browser-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --rustup-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-release-rustflags)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-engine-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-browser-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --git-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reload-server-host)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reload-server-port)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__new)
            opts="-t -h --template --dir --cargo-engine-path --cargo-browser-path --wasm-bindgen-path --wasm-opt-path --rustup-path --wasm-release-rustflags --cargo-engine-args --cargo-browser-args --wasm-bindgen-args --wasm-opt-args --git-path --reload-server-host --reload-server-port --sequential --no-browser-reload --wasm-bindgen-version --wasm-opt-version --no-system-tools-cache --verbose --disable-bundle-compression --help <NAME>"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --template)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -t)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --dir)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-engine-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-browser-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --rustup-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-release-rustflags)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-engine-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-browser-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --git-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reload-server-host)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reload-server-port)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__serve)
            opts="-w -h --no-run --no-build --release --standalone --watch --custom-watch --host --port --cargo-engine-path --cargo-browser-path --wasm-bindgen-path --wasm-opt-path --rustup-path --wasm-release-rustflags --cargo-engine-args --cargo-browser-args --wasm-bindgen-args --wasm-opt-args --git-path --reload-server-host --reload-server-port --sequential --no-browser-reload --wasm-bindgen-version --wasm-opt-version --no-system-tools-cache --verbose --disable-bundle-compression --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --custom-watch)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --host)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --port)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-engine-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-browser-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --rustup-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-release-rustflags)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-engine-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-browser-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --git-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reload-server-host)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reload-server-port)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__snoop)
            opts="-h --cargo-engine-path --cargo-browser-path --wasm-bindgen-path --wasm-opt-path --rustup-path --wasm-release-rustflags --cargo-engine-args --cargo-browser-args --wasm-bindgen-args --wasm-opt-args --git-path --reload-server-host --reload-server-port --sequential --no-browser-reload --wasm-bindgen-version --wasm-opt-version --no-system-tools-cache --verbose --disable-bundle-compression --help build wasm-build serve help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --cargo-engine-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-browser-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --rustup-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-release-rustflags)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-engine-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-browser-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --git-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reload-server-host)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reload-server-port)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__snoop__build)
            opts="-w -h --watch --custom-watch --cargo-engine-path --cargo-browser-path --wasm-bindgen-path --wasm-opt-path --rustup-path --wasm-release-rustflags --cargo-engine-args --cargo-browser-args --wasm-bindgen-args --wasm-opt-args --git-path --reload-server-host --reload-server-port --sequential --no-browser-reload --wasm-bindgen-version --wasm-opt-version --no-system-tools-cache --verbose --disable-bundle-compression --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --custom-watch)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-engine-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-browser-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --rustup-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-release-rustflags)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-engine-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-browser-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --git-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reload-server-host)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reload-server-port)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__snoop__help)
            opts="build wasm-build serve help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__snoop__help__build)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__snoop__help__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__snoop__help__serve)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__snoop__help__wasm__build)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 4 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__snoop__serve)
            opts="-w -h --host --port --watch --custom-watch --cargo-engine-path --cargo-browser-path --wasm-bindgen-path --wasm-opt-path --rustup-path --wasm-release-rustflags --cargo-engine-args --cargo-browser-args --wasm-bindgen-args --wasm-opt-args --git-path --reload-server-host --reload-server-port --sequential --no-browser-reload --wasm-bindgen-version --wasm-opt-version --no-system-tools-cache --verbose --disable-bundle-compression --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --host)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --port)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --custom-watch)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-engine-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-browser-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --rustup-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-release-rustflags)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-engine-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-browser-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --git-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reload-server-host)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reload-server-port)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__snoop__wasm__build)
            opts="-w -h --watch --custom-watch --cargo-engine-path --cargo-browser-path --wasm-bindgen-path --wasm-opt-path --rustup-path --wasm-release-rustflags --cargo-engine-args --cargo-browser-args --wasm-bindgen-args --wasm-opt-args --git-path --reload-server-host --reload-server-port --sequential --no-browser-reload --wasm-bindgen-version --wasm-opt-version --no-system-tools-cache --verbose --disable-bundle-compression --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --custom-watch)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-engine-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-browser-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --rustup-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-release-rustflags)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-engine-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-browser-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --git-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reload-server-host)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reload-server-port)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__test)
            opts="-w -h --no-build --show-browser --watch --custom-watch --host --port --cargo-engine-path --cargo-browser-path --wasm-bindgen-path --wasm-opt-path --rustup-path --wasm-release-rustflags --cargo-engine-args --cargo-browser-args --wasm-bindgen-args --wasm-opt-args --git-path --reload-server-host --reload-server-port --sequential --no-browser-reload --wasm-bindgen-version --wasm-opt-version --no-system-tools-cache --verbose --disable-bundle-compression --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --custom-watch)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --host)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --port)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-engine-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-browser-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --rustup-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-release-rustflags)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-engine-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-browser-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --git-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reload-server-host)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reload-server-port)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        perseus__tinker)
            opts="-h --no-clean --cargo-engine-path --cargo-browser-path --wasm-bindgen-path --wasm-opt-path --rustup-path --wasm-release-rustflags --cargo-engine-args --cargo-browser-args --wasm-bindgen-args --wasm-opt-args --git-path --reload-server-host --reload-server-port --sequential --no-browser-reload --wasm-bindgen-version --wasm-opt-version --no-system-tools-cache --verbose --disable-bundle-compression --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --cargo-engine-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-browser-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --rustup-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-release-rustflags)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-engine-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cargo-browser-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-args)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --git-path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reload-server-host)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reload-server-port)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-bindgen-version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wasm-opt-version)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

if [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 4 || "${BASH_VERSINFO[0]}" -gt 4 ]]; then
    complete -F _perseus -o nosort -o bashdefault -o default perseus
else
    complete -F _perseus -o bashdefault -o default perseus
fi
