# leading_workspace_flags_local

## `vp --filter app-a run hello`

--filter before the command

```
~/packages/app-a$ node -e "console.log('hello from app-a')" ⊘ cache disabled
hello from app-a
```

## `vp -w run hello`

-w before the command

```
$ node -e "console.log('hello from the root')" ⊘ cache disabled
hello from the root
```
