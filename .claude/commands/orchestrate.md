---
allowed-tools: [Bash]
description: "Autonomously orchestrate claude/ branches - merge ready PRs, resolve GitHub issues, consolidate overlapping work, and align repository with Faithful Archive vision"
argument-hint: "[--dry-run] [--aggressive] [--target-branch=main]"
---

# Claude Branch Orchestrator

You are an expert DevOps Product Manager with autonomous decision-making authority for the **Faithful Archive Dioxus App**. Your mission is to evolve the repository toward the documented vision while maintaining code quality and safety.

## Project Vision Context

**Faithful Archive Vision**: A Dioxus-based web application for uploading and sharing Christ-honoring spiritual content on Arweave's permanent storage. Core priorities:
1. **Dioxus Framework** with WebAssembly for high-performance web delivery
2. **Arweave Integration** - Permanent storage for spiritual content
3. **Content Moderation System** - Ensuring only Christ-honoring content is indexed
4. **Decentralized Archive** - Censorship-resistant spiritual content access
5. **Progressive Web App** - Offline functionality and mobile support

**Technical Standards**:
- Rust with Dioxus 0.6 framework
- WebAssembly (WASM) compilation target
- Service layer pattern (ArweaveService, WalletService, UploadService, StorageService)
- IndexedDB for local caching via rexie
- RSX syntax for component templates

## Enhanced Claude Branch Readiness Checks (Phase 3)

The orchestrator now performs comprehensive readiness assessments specifically for Claude branches before merging into master:

1. **Merge Conflict Detection**: Tests if the branch can merge cleanly
2. **Code Quality Validation**: Checks for TODO/FIXME comments and print() statements
3. **Test Coverage**: Verifies test files exist for new implementation files
4. **Branch Freshness**: Ensures branch isn't too far behind master
5. **Issue Resolution**: Detects which GitHub issues are resolved

**Readiness Scoring** (out of 9 points):
- No merge conflicts: +3 points
- Clean code quality: +2 points  
- Adequate test coverage: +1 point
- Branch freshness: +1-2 points

**Merge Decision Criteria**:
- Normal mode: Requires total score ≥15/19 with no readiness issues
- Aggressive mode: Accepts total score ≥10/19 with ≤1 minor issue
- Recent branches (<24h) get priority consideration

## Phase 1: Discovery & State Assessment

```bash
echo "=== SANCTILY BRANCH ORCHESTRATION STARTING ==="
echo "Timestamp: $(date -Iseconds)"

# Parse execution arguments
DRY_RUN=""
AGGRESSIVE=""
TARGET_BRANCH="main"

if [[ "$ARGUMENTS" == *"--dry-run"* ]]; then
    DRY_RUN="true"
    echo "🔍 DRY-RUN MODE: No changes will be made"
fi

if [[ "$ARGUMENTS" == *"--aggressive"* ]]; then
    AGGRESSIVE="true"
    echo "⚡ AGGRESSIVE MODE: Will merge without full reviews"
fi

if [[ "$ARGUMENTS" == *"--target-branch="* ]]; then
    TARGET_BRANCH=$(echo "$ARGUMENTS" | sed -n 's/.*--target-branch=\([^ ]*\).*/\1/p')
fi

echo "Target branch: $TARGET_BRANCH"
echo ""

# Ensure we have latest state
git fetch --all --prune

# Discover all claude/ branches
echo "=== CLAUDE BRANCH DISCOVERY ==="
claude_branches=($(git branch -r | grep "origin/claude/" | sed 's/origin\///' | head -20))

if [ ${#claude_branches[@]} -eq 0 ]; then
    echo "ℹ️  No claude/ branches found. Repository is clean!"
    exit 0
fi

echo "Found ${#claude_branches[@]} claude/ branches:"
for branch in "${claude_branches[@]}"; do
    last_commit=$(git log -1 --format="%cd - %s" --date=short origin/$branch 2>/dev/null)
    commits_ahead=$(git rev-list --count origin/$TARGET_BRANCH..origin/$branch 2>/dev/null || echo "0")
    echo "  📋 $branch ($commits_ahead commits ahead)"
    echo "      Last: $last_commit"
done
echo ""
```

## Phase 2: GitHub Issue Integration & Analysis

```bash
echo "=== GITHUB ISSUE DISCOVERY ==="

# Check if gh CLI is available
if ! command -v gh &> /dev/null; then
    echo "⚠️  GitHub CLI not found. Install with: brew install gh"
    echo "   Issue resolution tracking will be limited to commit message parsing"
    echo "   Install with: brew install gh && gh auth login"
    HAS_GH=false
else
    HAS_GH=true
    echo "✅ GitHub CLI available - enabling full issue integration"
fi

# Discover open issues if GitHub CLI is available
declare -A issue_map=()
declare -A issue_priority=()
declare -A issue_labels=()

if [ "$HAS_GH" = "true" ]; then
    echo ""
    echo "📋 Fetching open GitHub issues..."
    
    # Get open issues with labels and priorities
    issues_json=$(gh issue list --state open --json number,title,labels,assignees,createdAt --limit 50 2>/dev/null || echo "[]")
    
    if [ "$issues_json" != "[]" ] && [ -n "$issues_json" ]; then
        echo "Found open issues:"
        
        # Parse issues (simplified approach for bash)
        issue_numbers=$(echo "$issues_json" | grep -o '"number":[0-9]*' | cut -d':' -f2 | head -20)
        
        for issue_num in $issue_numbers; do
            # Get issue details
            issue_title=$(gh issue view $issue_num --json title --jq '.title' 2>/dev/null || echo "Unknown")
            issue_labels_raw=$(gh issue view $issue_num --json labels --jq '.labels[].name' 2>/dev/null | tr '\n' ',' | sed 's/,$//')
            
            # Store issue information
            issue_map[$issue_num]="$issue_title"
            issue_labels[$issue_num]="$issue_labels_raw"
            
            # Determine priority from labels
            priority="LOW"
            if echo "$issue_labels_raw" | grep -qi "urgent\|critical\|high"; then
                priority="HIGH"
            elif echo "$issue_labels_raw" | grep -qi "medium\|important"; then
                priority="MEDIUM"
            fi
            issue_priority[$issue_num]="$priority"
            
            echo "  🐛 #$issue_num [$priority] $issue_title"
            [ -n "$issue_labels_raw" ] && echo "      Labels: $issue_labels_raw"
        done
    else
        echo "  ℹ️  No open issues found or unable to fetch"
    fi
else
    echo "  ⚠️  Skipping GitHub issue discovery (gh CLI not available)"
fi

echo ""
```

## Phase 3: Claude Branch Readiness & Auto-Merge

```bash
echo "=== CLAUDE BRANCH READINESS ASSESSMENT ==="

merged_count=0
skipped_count=0
failed_checks=0

# Specifically look for claude/ branches only
claude_branches=($(git branch -r | grep "origin/claude/" | sed 's/origin\///' | head -20))

echo "Found ${#claude_branches[@]} claude/ branches to evaluate for merge into $TARGET_BRANCH"
echo ""

for branch in "${claude_branches[@]}"; do
    echo "🔍 Analyzing branch: $branch"
    
    # Check if branch exists and get basic info
    if ! git rev-parse --verify origin/$branch >/dev/null 2>&1; then
        echo "  ⚠️  Branch origin/$branch not found, skipping"
        continue
    fi
    
    # Basic branch metrics
    commits_ahead=$(git rev-list --count origin/$TARGET_BRANCH..origin/$branch 2>/dev/null || echo "0")
    commits_behind=$(git rev-list --count origin/$branch..origin/$TARGET_BRANCH 2>/dev/null || echo "0")
    last_activity=$(git log -1 --format="%ct" origin/$branch 2>/dev/null || echo "0")
    current_time=$(date +%s)
    days_ago=$(( ($current_time - $last_activity) / 86400 ))
    hours_ago=$(( ($current_time - $last_activity) / 3600 ))
    
    # Get changed files for analysis
    changed_files=$(git diff --name-only origin/$TARGET_BRANCH...origin/$branch 2>/dev/null || echo "")
    changed_files_count=$(echo "$changed_files" | grep -v "^$" | wc -l | tr -d ' ')
    
    echo "  📊 Branch Metrics:"
    echo "     • Commits ahead: $commits_ahead | behind: $commits_behind"
    echo "     • Files changed: $changed_files_count"
    echo "     • Last activity: $([ $days_ago -gt 0 ] && echo "${days_ago}d ago" || echo "${hours_ago}h ago")"
    
    # Perform readiness checks
    readiness_score=0
    readiness_issues=()
    
    # Check 1: Test for merge conflicts
    echo "  🧪 Running merge conflict test..."
    git checkout -q $TARGET_BRANCH 2>/dev/null
    git pull -q origin $TARGET_BRANCH 2>/dev/null
    
    if git merge --no-commit --no-ff origin/$branch >/dev/null 2>&1; then
        echo "     ✅ No merge conflicts detected"
        readiness_score=$((readiness_score + 3))
        git merge --abort 2>/dev/null
    else
        echo "     ❌ Merge conflicts detected"
        readiness_issues+=("merge conflicts")
        git merge --abort 2>/dev/null
    fi
    
    # Check 2: Code quality validation (Rust/Dioxus specific)
    echo "  🔍 Checking code quality..."
    
    # Check for common Rust/Dioxus issues in changed files
    rust_issues=0
    if echo "$changed_files" | grep -q "\.rs$"; then
        # Check for TODO comments in new code
        new_todos=$(git diff origin/$TARGET_BRANCH...origin/$branch | grep "^+" | grep -c "TODO\|FIXME\|XXX" || echo "0")
        if [ $new_todos -gt 0 ]; then
            echo "     ⚠️  Found $new_todos new TODO/FIXME comments"
            readiness_issues+=("$new_todos unresolved TODOs")
        fi
        
        # Check for println! statements (should use log::info! or similar)
        new_prints=$(git diff origin/$TARGET_BRANCH...origin/$branch | grep "^+" | grep -c "println!(" || echo "0")
        if [ $new_prints -gt 0 ]; then
            echo "     ⚠️  Found $new_prints println!() statements (use log macros instead)"
            rust_issues=$((rust_issues + new_prints))
        fi
        
        # Check for unwrap() calls (should use proper error handling)
        new_unwraps=$(git diff origin/$TARGET_BRANCH...origin/$branch | grep "^+" | grep -c "\.unwrap()" || echo "0")
        if [ $new_unwraps -gt 5 ]; then
            echo "     ⚠️  Found $new_unwraps .unwrap() calls (consider proper error handling)"
            rust_issues=$((rust_issues + 1))
        fi
    fi
    
    if [ $rust_issues -eq 0 ]; then
        echo "     ✅ Code quality checks passed"
        readiness_score=$((readiness_score + 2))
    else
        readiness_issues+=("$rust_issues code quality issues")
    fi
    
    # Check 3: Test file coverage
    echo "  📋 Checking test coverage..."
    test_files_added=$(echo "$changed_files" | grep -c "test.*\.rs$\|tests/.*\.rs$" || echo "0")
    implementation_files_added=$(echo "$changed_files" | grep -c "src/.*\.rs$" || echo "0")
    
    if [ $implementation_files_added -gt 2 ] && [ $test_files_added -eq 0 ]; then
        echo "     ⚠️  No tests added for $implementation_files_added implementation files"
        readiness_issues+=("missing tests")
    else
        echo "     ✅ Test coverage acceptable"
        readiness_score=$((readiness_score + 1))
    fi
    
    # Check 4: Branch freshness
    if [ $commits_behind -gt 10 ]; then
        echo "  ⚠️  Branch is $commits_behind commits behind $TARGET_BRANCH"
        readiness_issues+=("$commits_behind commits behind")
    elif [ $commits_behind -gt 0 ]; then
        echo "  ℹ️  Branch is $commits_behind commits behind $TARGET_BRANCH (acceptable)"
        readiness_score=$((readiness_score + 1))
    else
        echo "  ✅ Branch is up to date with $TARGET_BRANCH"
        readiness_score=$((readiness_score + 2))
    fi
    
    # Check 5: Issue resolution detection
    echo "  🎯 Checking for issue resolutions..."
    resolved_issues=()
    commit_messages=$(git log --format="%s %b" origin/$TARGET_BRANCH..origin/$branch 2>/dev/null || echo "")
    
    # Check for issue references in commit messages
    if [ -n "$commit_messages" ]; then
        # Look for common issue reference patterns
        issue_refs=$(echo "$commit_messages" | grep -io -E "(fix|fixes|fixed|close|closes|closed|resolve|resolves|resolved)\s*#?[0-9]+" | grep -o -E "[0-9]+" | sort -u)
        
        for issue_ref in $issue_refs; do
            # Verify this is actually an open issue
            if [ "$HAS_GH" = "true" ] && [ -n "${issue_map[$issue_ref]:-}" ]; then
                resolved_issues+=($issue_ref)
                echo "  🎯 RESOLVES ISSUE #$issue_ref: ${issue_map[$issue_ref]}"
                echo "      Priority: ${issue_priority[$issue_ref]:-UNKNOWN}"
            elif echo "$commit_messages" | grep -qi "issue.*$issue_ref\|#$issue_ref"; then
                # Issue reference found but can't verify - still note it
                resolved_issues+=($issue_ref)
                echo "  🎯 REFERENCES ISSUE #$issue_ref (unable to verify details)"
            fi
        done
    fi
    
    # Calculate vision alignment score (1-10)
    vision_score=5  # Default neutral score
    
    # Boost score for issue resolution
    if [ ${#resolved_issues[@]} -gt 0 ]; then
        for issue_ref in "${resolved_issues[@]}"; do
            case "${issue_priority[$issue_ref]:-LOW}" in
                "HIGH")   vision_score=$((vision_score + 3)) ;;
                "MEDIUM") vision_score=$((vision_score + 2)) ;;
                "LOW")    vision_score=$((vision_score + 1)) ;;
            esac
        done
        echo "  ✅ Issue resolution bonus: +$((${#resolved_issues[@]} * 2)) points"
    fi
    
    # Boost score for vision-aligned changes
    if echo "$changed_files" | grep -q "src/services/.*\.rs"; then
        vision_score=$((vision_score + 2))  # Service layer improvements
    fi
    if echo "$changed_files" | grep -q "src/components/.*\.rs"; then
        vision_score=$((vision_score + 1))  # Dioxus component work
    fi
    if echo "$changed_files" | grep -q "arweave\|Arweave"; then
        vision_score=$((vision_score + 2))  # Blockchain integration
    fi
    if echo "$changed_files" | grep -q "assets/.*\.css\|src/.*theme"; then
        vision_score=$((vision_score + 1))  # Styling improvements
    fi
    if echo "$changed_files" | grep -q "Cargo.toml\|Dioxus.toml"; then
        vision_score=$((vision_score + 1))  # Build configuration
    fi
    
    # Reduce score for concerning patterns
    if echo "$changed_files" | grep -q "Cargo.toml" && [ $commits_ahead -gt 10 ]; then
        vision_score=$((vision_score - 1))  # Large dependency changes
    fi
    
    # Cap the score
    if [ $vision_score -gt 10 ]; then vision_score=10; fi
    if [ $vision_score -lt 1 ]; then vision_score=1; fi
    
    echo "  📊 Vision Alignment Score: $vision_score/10"
    echo "  ✅ Readiness Score: $readiness_score/9"
    
    # Calculate total merge confidence
    total_score=$((vision_score + readiness_score))
    echo "  🎯 Total Merge Confidence: $total_score/19"
    
    # Show any readiness issues
    if [ ${#readiness_issues[@]} -gt 0 ]; then
        echo "  ⚠️  Readiness Issues:"
        for issue in "${readiness_issues[@]}"; do
            echo "     • $issue"
        done
    fi
    
    # Determine if branch should be auto-merged
    should_merge=false
    merge_reason=""
    
    if [ $commits_ahead -eq 0 ]; then
        echo "  ✅ Branch is fully merged - candidate for cleanup"
        should_merge="cleanup"
        merge_reason="fully merged"
    elif [ ${#readiness_issues[@]} -eq 0 ] && [ $total_score -ge 15 ] && [ $commits_ahead -le 5 ]; then
        # Perfect readiness + high vision alignment + small scope
        should_merge=true
        merge_reason="excellent readiness ($readiness_score/9), high vision alignment ($vision_score/10)"
    elif [ ${#readiness_issues[@]} -eq 0 ] && [ $total_score -ge 12 ] && [ $hours_ago -le 24 ]; then
        # Perfect readiness + good scores + very recent
        should_merge=true
        merge_reason="no issues, good scores ($total_score/19), very recent activity"
    elif [ "$AGGRESSIVE" = "true" ] && [ $total_score -ge 10 ] && [ ${#readiness_issues[@]} -le 1 ]; then
        # Aggressive mode: accept minor issues if scores are decent
        should_merge=true
        merge_reason="aggressive mode: acceptable scores ($total_score/19), minor issues only"
    elif [ ${#readiness_issues[@]} -gt 0 ]; then
        # Don't merge if there are readiness issues in normal mode
        should_merge=false
        merge_reason="readiness issues: ${readiness_issues[*]}"
    fi
    
    # Execute merge decision
    if [ "$should_merge" = "cleanup" ]; then
        if [ "$DRY_RUN" = "true" ]; then
            echo "  🔍 DRY-RUN: Would delete fully merged branch $branch"
        else
            echo "  🗑️  DELETING fully merged branch: $branch"
            git push origin --delete $branch 2>/dev/null || echo "    (branch already deleted)"
            merged_count=$((merged_count + 1))
        fi
    elif [ "$should_merge" = "true" ]; then
        if [ "$DRY_RUN" = "true" ]; then
            echo "  🔍 DRY-RUN: Would merge $branch ($merge_reason)"
        else
            echo "  ✅ MERGING $branch: $merge_reason"
            
            # Create merge commit with detailed readiness info
            merge_message="feat: merge $branch

Merge reason: $merge_reason
Vision alignment: $vision_score/10
Readiness score: $readiness_score/9
Total confidence: $total_score/19

Branch metrics:
- Commits: $commits_ahead ahead, $commits_behind behind
- Files changed: $changed_files_count
- Last activity: $([ $days_ago -gt 0 ] && echo "${days_ago}d ago" || echo "${hours_ago}h ago")"

            # Add resolved issues to merge message
            if [ ${#resolved_issues[@]} -gt 0 ]; then
                merge_message="$merge_message

Resolves: $(printf "#%s " "${resolved_issues[@]}")"
            fi

            merge_message="$merge_message

🤖 Auto-merged by Claude Code Orchestrator"

            if git checkout $TARGET_BRANCH && git pull origin $TARGET_BRANCH; then
                if git merge origin/$branch --no-ff -m "$merge_message"; then
                    git push origin $TARGET_BRANCH
                    
                    # Close resolved issues if GitHub CLI is available
                    if [ "$HAS_GH" = "true" ] && [ ${#resolved_issues[@]} -gt 0 ]; then
                        echo "    🎯 Closing resolved issues..."
                        for issue_ref in "${resolved_issues[@]}"; do
                            if [ -n "${issue_map[$issue_ref]:-}" ]; then
                                gh issue close $issue_ref --comment "Automatically closed by merge of $branch via Claude Code Orchestrator.

This issue was resolved in the merged changes." 2>/dev/null && echo "      ✅ Closed issue #$issue_ref" || echo "      ⚠️  Failed to close issue #$issue_ref"
                            fi
                        done
                    fi
                    
                    git push origin --delete $branch 2>/dev/null || echo "    (branch deletion failed, may need manual cleanup)"
                    merged_count=$((merged_count + 1))
                    echo "    ✅ Successfully merged and deleted $branch"
                else
                    echo "    ❌ Merge failed - keeping branch for manual review"
                    git merge --abort 2>/dev/null
                    skipped_count=$((skipped_count + 1))
                fi
            else
                echo "    ❌ Failed to prepare $TARGET_BRANCH for merge"
                skipped_count=$((skipped_count + 1))
            fi
        fi
    else
        echo "  ⏸️  SKIPPED: $branch (score: $vision_score, commits: $commits_ahead, age: ${days_ago}d)"
        skipped_count=$((skipped_count + 1))
    fi
    
    echo ""
done
```

## Phase 4: Branch Consolidation Analysis

```bash
echo "=== BRANCH CONSOLIDATION ANALYSIS ==="

# Re-discover remaining branches after merges
remaining_branches=($(git branch -r | grep "origin/claude/" | sed 's/origin\///' | head -20))

if [ ${#remaining_branches[@]} -le 1 ]; then
    echo "ℹ️  Only ${#remaining_branches[@]} claude/ branch(es) remaining - no consolidation needed"
else
    echo "Analyzing ${#remaining_branches[@]} remaining branches for consolidation opportunities..."
    
    # Check for overlapping file changes between branches
    for i in "${!remaining_branches[@]}"; do
        branch1="${remaining_branches[$i]}"
        
        for j in "${!remaining_branches[@]}"; do
            if [ $i -ge $j ]; then continue; fi  # Skip self and already compared pairs
            
            branch2="${remaining_branches[$j]}"
            
            # Get files changed in each branch
            files1=$(git diff --name-only origin/$TARGET_BRANCH...origin/$branch1 2>/dev/null | sort)
            files2=$(git diff --name-only origin/$TARGET_BRANCH...origin/$branch2 2>/dev/null | sort)
            
            # Find overlapping files
            overlap_count=0
            if [ -n "$files1" ] && [ -n "$files2" ]; then
                overlap_count=$(comm -12 <(echo "$files1") <(echo "$files2") | wc -l | tr -d ' ')
            fi
            
            if [ $overlap_count -gt 0 ]; then
                echo "  🔄 OVERLAP DETECTED: $branch1 ↔ $branch2 ($overlap_count shared files)"
                
                # Analyze which branch is more developed
                commits1=$(git rev-list --count origin/$TARGET_BRANCH..origin/$branch1 2>/dev/null || echo "0")
                commits2=$(git rev-list --count origin/$TARGET_BRANCH..origin/$branch2 2>/dev/null || echo "0")
                
                # Determine primary branch (more commits = primary)
                if [ $commits1 -gt $commits2 ]; then
                    primary_branch="$branch1"
                    secondary_branch="$branch2"
                else
                    primary_branch="$branch2"
                    secondary_branch="$branch1"
                fi
                
                echo "    Suggested consolidation: $secondary_branch → $primary_branch"
                
                if [ "$DRY_RUN" = "true" ]; then
                    echo "    🔍 DRY-RUN: Would consolidate $secondary_branch into $primary_branch"
                else
                    echo "    🔄 CONSOLIDATING: $secondary_branch → $primary_branch"
                    
                    # Perform consolidation
                    if git checkout origin/$primary_branch -b temp-consolidate-$primary_branch; then
                        if git merge origin/$secondary_branch --no-ff -m "consolidate: merge $secondary_branch into $primary_branch

Consolidation reason: $overlap_count overlapping files
Combined vision alignment for Sanctily development

🤖 Auto-consolidated by Claude Code Orchestrator"; then
                            # Push consolidated branch
                            git push origin temp-consolidate-$primary_branch:$primary_branch --force
                            
                            # Clean up
                            git checkout $TARGET_BRANCH
                            git branch -D temp-consolidate-$primary_branch
                            git push origin --delete $secondary_branch
                            
                            echo "    ✅ Successfully consolidated $secondary_branch into $primary_branch"
                            
                            # Remove consolidated branch from remaining list
                            remaining_branches=($(printf '%s\n' "${remaining_branches[@]}" | grep -v "^$secondary_branch$"))
                        else
                            echo "    ❌ Consolidation merge failed - keeping branches separate"
                            git merge --abort 2>/dev/null
                            git checkout $TARGET_BRANCH
                            git branch -D temp-consolidate-$primary_branch 2>/dev/null
                        fi
                    else
                        echo "    ❌ Failed to checkout consolidation branch"
                    fi
                fi
            fi
        done
    done
fi
```

## Phase 5: Strategic Assessment & Next Steps

```bash
echo "=== SANCTILY DEVELOPMENT STATUS ==="

# Final repository state
final_claude_branches=($(git branch -r | grep "origin/claude/" | sed 's/origin\///' | head -20))
total_remaining=${#final_claude_branches[@]}

# Count resolved issues
total_issues_resolved=0
if [ "$HAS_GH" = "true" ]; then
    # This is a simplified count - in practice you'd track this during merging
    closed_issues=$(gh issue list --state closed --limit 20 --json number,closedAt --jq '.[] | select(.closedAt | fromdateiso8601 > (now - 3600)) | .number' 2>/dev/null | wc -l | tr -d ' ')
    total_issues_resolved=$closed_issues
fi

echo "📊 Orchestration Results:"
echo "    ✅ Branches merged/cleaned: $merged_count"
echo "    ⏸️  Branches skipped: $skipped_count" 
echo "    🔄 Branches remaining: $total_remaining"
echo "    🎯 Issues resolved: $total_issues_resolved"
echo ""

if [ $total_remaining -gt 0 ]; then
    echo "🎯 Remaining Claude Branches - Next Priority Assessment:"
    
    for branch in "${final_claude_branches[@]}"; do
        # Re-analyze remaining branches for priority
        commits_ahead=$(git rev-list --count origin/$TARGET_BRANCH..origin/$branch 2>/dev/null || echo "0")
        changed_files=$(git diff --name-only origin/$TARGET_BRANCH...origin/$branch 2>/dev/null)
        last_activity=$(git log -1 --format="%ct" origin/$branch 2>/dev/null || echo "0")
        current_time=$(date +%s)
        days_ago=$(( ($current_time - $last_activity) / 86400 ))
        
        # Priority assessment for Sanctily vision
        priority="LOW"
        priority_reason="needs review"
        
        if echo "$changed_files" | grep -q "src/services/arweave\.rs\|src/services/wallet\.rs"; then
            priority="HIGH"
            priority_reason="core service implementation"
        elif echo "$changed_files" | grep -q "src/components/.*\.rs"; then
            priority="MEDIUM"
            priority_reason="user interface improvements"
        elif echo "$changed_files" | grep -q "Cargo.toml\|Dioxus.toml" && [ $commits_ahead -le 3 ]; then
            priority="MEDIUM"
            priority_reason="dioxus framework updates"
        elif [ $days_ago -le 3 ] && [ $commits_ahead -le 5 ]; then
            priority="MEDIUM"
            priority_reason="recent active development"
        fi
        
        echo "    📋 $branch - Priority: $priority"
        echo "        Reason: $priority_reason"
        echo "        Activity: $days_ago days ago, $commits_ahead commits"
        echo ""
    done
    
    echo "🚀 Recommended Next Actions:"
    echo "1. Review HIGH priority branches for immediate manual merge"
    echo "2. Test MEDIUM priority branches in development environment" 
    echo "3. Consider archiving LOW priority branches if inactive >30 days"
    echo ""
    
    echo "💡 Faithful Archive Vision Alignment Tips:"
    echo "   - Prioritize ArweaveService & WalletService completion"
    echo "   - Focus on Upload/Browse Dioxus component polish"
    echo "   - Ensure WASM optimization for web performance"
    echo "   - Maintain spiritual content moderation workflow"
    echo ""
    
    echo "🔗 GitHub Integration Status:"
    if [ "$HAS_GH" = "true" ]; then
        echo "   ✅ GitHub CLI available - full issue integration enabled"
        echo "   ✅ Issues automatically closed when branches merge"
        echo "   ✅ Issue priorities boost merge scoring"
    else
        echo "   ⚠️  GitHub CLI not available - limited issue detection"
        echo "   💡 Install with: brew install gh && gh auth login"
        echo "   💡 Enable full issue integration for better automation"
    fi
else
    echo "🎉 All claude/ branches have been successfully orchestrated!"
    echo "Repository is now fully aligned with Sanctily development vision."
fi

echo ""
echo "=== ORCHESTRATION COMPLETE ==="
echo "Execution mode: $([ "$DRY_RUN" = "true" ] && echo "DRY-RUN" || echo "LIVE")"
echo "Target branch: $TARGET_BRANCH"
echo "Completed at: $(date -Iseconds)"

# Return to main branch
git checkout $TARGET_BRANCH 2>/dev/null || true
```

## Safety Validations

This orchestrator includes multiple safety mechanisms:

1. **DRY-RUN Mode**: `--dry-run` shows what would happen without making changes
2. **Vision Scoring**: Only merges branches aligned with Sanctily's goals  
3. **GitHub Issue Integration**: Automatically resolves issues and boosts merge priority
4. **Size Limits**: Avoids auto-merging large, complex changes
5. **Recency Checks**: Prioritizes active development over stale work
6. **Merge Validation**: Aborts failed merges and preserves branches
7. **Comprehensive Logging**: Full audit trail of all decisions

## Usage Examples

- **Safe Preview**: `/orchestrate --dry-run`
- **Standard Mode**: `/orchestrate` 
- **Aggressive Mode**: `/orchestrate --aggressive`
- **Custom Target**: `/orchestrate --target-branch=develop --dry-run`

The orchestrator autonomously evolves your Sanctily repository toward the documented vision while maintaining safety through configurable execution modes and comprehensive validation.

## GitHub Issue Integration

The enhanced orchestrator now includes full GitHub issue integration when the GitHub CLI (`gh`) is available:

- **🔍 Issue Discovery**: Automatically fetches open issues with priorities and labels
- **🎯 Resolution Detection**: Scans commit messages for issue references (fixes #123, closes #456, etc.)
- **📊 Priority Scoring**: Issues with "high" or "urgent" labels boost merge scores significantly  
- **🔒 Automatic Closure**: Successfully merged branches automatically close referenced issues
- **📝 Audit Trail**: All issue resolutions are documented in merge commits

**Setup GitHub Integration:**
```bash
# Install GitHub CLI
brew install gh

# Authenticate with GitHub
gh auth login

# Verify integration
gh issue list --limit 5
```

**Issue Reference Patterns Detected:**
- `fixes #123`, `fix #123`, `fixed #123`  
- `closes #123`, `close #123`, `closed #123`
- `resolves #123`, `resolve #123`, `resolved #123`
- Direct references: `issue #123`, `#123`