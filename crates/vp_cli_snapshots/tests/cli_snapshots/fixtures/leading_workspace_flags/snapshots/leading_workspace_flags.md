# leading_workspace_flags

## `vp --filter app-a run hello`

--filter before the command

```
VITE+ - The Unified Toolchain for the Web

~/packages/app-a$ node -e "console.log('hello from app-a')" ⊘ cache disabled
hello from app-a
```

## `vp -w run hello`

-w before the command

```
VITE+ - The Unified Toolchain for the Web

$ node -e "console.log('hello from the root')" ⊘ cache disabled
hello from the root
```

## `vp -F app-b exec -- node -e console.log(process.env.VP_PACKAGE_NAME)`

-F before exec

```
VITE+ - The Unified Toolchain for the Web

app-b
```

## `vp run nested`

the same order inside a script

```
VITE+ - The Unified Toolchain for the Web

~/packages/app-a$ node -e "console.log('hello from app-a')" ⊘ cache disabled
hello from app-a
```

## `vp -r fmt`

tool commands do not take workspace flags

**Exit code:** 2

```
VITE+ - The Unified Toolchain for the Web

error: Unexpected argument '-r'
```
