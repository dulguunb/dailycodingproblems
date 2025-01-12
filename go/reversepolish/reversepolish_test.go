package reversepolish

import "testing"

func Test_ReversePolish(t *testing.T) {
	tests := []struct {
		name        string
		symbols     []string
		expected    int
		expectError bool
	}{

		{
			"Input from the problem",
			[]string{"15", "7", "1", "1", "+", "-", "/", "3", "*", "2", "1", "1", "+", "+", "-"},
			5,
			false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result, err := ReversePolish(tt.symbols)
			if tt.expectError {
				t.Log(err)
			}
			if result != tt.expected && !tt.expectError {
				t.Errorf("Polish(%s) = %d; want %d", tt.symbols, result, tt.expected)
			}
		})
	}
}
