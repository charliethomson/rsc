
tokenType = oneof
    'LeftParen',
    'RightParen',
    'OperatorAdd',
    'OperatorMul',
    'OperatorDiv',
    'OperatorSub',
    'Value'

token = type {
    literal: string;
    type: tokenType;
    value: number?;

    new = static (.literal, .type, .value): token;

    createValue = static (value: string): token? => {
        // If there are no items in `value` that aren't digits, it won't return here
        if (!value.isDigit()) { return nil }

        return token.new(value, "Value", number.fromString(value))
    }

    createOperator = static (operator: string): token? => when {
        operator == '+' => token.new(operator, 'OperatorAdd')
        operator == '-' => token.new(operator, 'OperatorSub')
        operator == '*' => token.new(operator, 'OperatorMul')
        operator == '/' => token.new(operator, 'OperatorDiv')
    } else nil

    createParen = static (paren: string): token => when {
        paren == '(' => token.new(paren, 'LeftParen')
        paren == ')' => token.new(paren, 'RightParen')
    } else nil
}

characterClassification = oneof 
        'operator',
        'numeric',
        'paren',
        'white'

classifyCharacter = (char: string): characterClassification = when {
    char in '+-*/' => 'operator',
    char in '()' => 'paren',
    char in [0, 9] => 'numeric',
} else 'white'

scanner = type {
    buffer: string;
    idx: number = 0;
    current: token? = nil;
    previous: token[] = [];

    new = static (.buffer): scanner;


    scan = (): bool => {
        if (.current != nil) .previous.add(.current);

        if (.index >= .buffer.length) return false;

        classification = classifyCharacter(.buffer[.index])
        start = .index

        .current = when {
            classification == 'white' => {
                while (.index < .buffer.length && classifyCharacter(.buffer[.index]) == 'white') .index++;

                return !(.index == .buffer.length)
            }
            classification == 'numeric' => {
                while (.index < .buffer.length && classifyCharacter(.buffer[.index]) == 'numeric') .index++;

                if (.index == .buffer.length) return false;

                token.createValue(.buffer[start, .index))
            }
            classification == 'operator' => token.createOperator(.buffer[.index++])
            classification == 'paren' => token.createParen(.buffer[.index++])
        }
    }
}

s = scanner.new("(1 + 2) / 3");
while (s.scan());

for token = s.previous.each => print(token);