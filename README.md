# Yaoya

Negicloud acccount management platform

## Workflow

### Signup with default(Normal committee-member) role

```mermaid
sequenceDiagram
    participant slack
    participant yaoya
    participant negicloud
    participant email
    slack->>yaoya: \signup
    opt User registerd with non-s-address
        yaoya ->> slack: You're not registerd with s-address. Enter your email.
        slack ->> yaoya: Email address
        yaoya ->> email: Verification link
        browser ->> yaoya: Verification Request
    end
    yaoya->>slack: Your email is confirmed. You're trying to singup with s#######@s.tsukuba.ac.jp. Base on your department/year/name has been parsed like (foo). Is it correct?
    alt correct
        slack ->> yaoya: Yes
        yaoya ->> slack: No
    else incorrect
        slack ->> yaoya: No
        yaoya ->> slack: Tell me your correct department/year/name
        slack ->> yaoya: department/year/name
        yaoya ->> slack: Your correct username is XXXXXXXX. Fix and try again.
    end
    yaoya->>negicloud: create user
    yaoya->>slack: default password
```

